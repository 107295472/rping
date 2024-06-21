use crate::{
    commons::{common, consts, get_config::CFG, model::AlctAPIModel},
    send_api,
};
use redis::Commands;
use rust_xlsxwriter::{Format, Workbook};
use serde::{Deserialize, Serialize};
pub async fn invoice_details_gen(fph: String) {
    // let mut hashmap: HashMap<Option<String>, Option<String>> = HashMap::new();
    let mut resultlist: Vec<ListByIdsResult> = Vec::new();
    let mut con = common::get_redis().unwrap();
    let dd = con.get(consts::TOKEN);
    let para=format!("freight-tax/invoice/anfInvoiceInfo/list?_t={}&fpsqh={}&column=createTime&order=desc&field=id,,fpsqh,fpsqStatus_dictText,xfmc,xfhyh,xfnsrsbh,kpje,createTime,sqdhm,taxSyncDate,isDown,action&pageNo=1&pageSize=10",common::get_timestamp(true),fph);
    // let para=format!("freight-tax/complain/anfComplain/ssList?_t={}&column=createTime&order=desc&field=id,,kpStatus,auditFlag,shipmentCode,auditReason,action&pageNo={}&pageSize=30",common::get_timestamp(true),n+1);
    match dd {
        Ok(token) => {
            // dbg!(t);
            // let t: LocalToken = serde_json::from_str(&token).unwrap();
            let mut req = AlctAPIModel {
                menthod: "get".to_string(),
                token: Some(token),
                method_adress: para,
                method_domain: CFG.system.method_domain.clone(),
                data: String::default(),
            };
            let resdata = send_api(req.clone()).await.unwrap();
            // let ress = match resdata {
            //     Ok(v) => v,
            //     Err(e) => panic!("{}", e.to_string()),
            // };
            let res: InvoiceDetailsRoot = serde_json::from_str(&resdata).unwrap();
            // let db = Database::connect("sqlite::memory:")
            //     .await
            //     .expect("数据库初始错误");
            // println!("code:{},for index:{}", res.code.unwrap(), n);
            let search_records: Option<Vec<InvoiceDetailsRecord>> = res.result.unwrap().records;
            // println!("{},{},{}", n, records.is_none(), records.unwrap().len());
            // for i in records.unwrap().into_iter() {
            //     // println!("{}", i.kp_status.clone().unwrap());
            //     // hashmap.insert(i.shipment_code.clone(), i.kp_status.clone());
            //     result.push(i);
            // }
            common::sleep_async(1).await;
            if search_records.is_some() {
                let mut yids = search_records.unwrap();
                let ids = yids[0].ydids.as_mut().unwrap();
                req.method_adress = format!(
                    "freight-tax/waybillInfo/waybillInfo/listByIds?_t={}&ydid=&ydids={}",
                    common::get_timestamp(true),
                    ids
                );
                let liststr = send_api(req).await.unwrap();
                // println!("{}", liststr);
                // log::info!("{}", liststr);
                let reslist: ListByIdsRoot = serde_json::from_str(&liststr).unwrap();
                // println!("{:?}", reslist);
                for i in reslist.result.unwrap().into_iter() {
                    // println!("{}", i.kp_status.clone().unwrap());
                    // hashmap.insert(i.shipment_code.clone(), i.kp_status.clone());
                    resultlist.push(i);
                }
            }
        }
        Err(e) => panic!("token不存在:{}", e),
    };
    common::sleep_async(1).await;
    gen_excel(resultlist).await;
}
async fn gen_excel(m: Vec<ListByIdsResult>) {
    // println!("len---{}", m.len());
    // log::info!("len--{}", m.len());
    let mut workbook = Workbook::new();
    let _worksheet = workbook.add_worksheet();
    _ = _worksheet.set_name("Sheet0");
    // global::info_msg("2222222222");
    let colv = vec!["运单编号", "上次审核标记", "审核意见"];
    let mut index = 0;
    // let count = colv.len();
    for i in colv {
        _ = _worksheet.write_string_with_format(0, index, i, &Format::new().set_bold());
        _ = _worksheet.set_column_width(index, 26);
        index += 1;
    }
    // _ = _worksheet.write_string_with_format(0, count as u16, "状态", &Format::new().set_bold());
    // _ = _worksheet.set_column_width(index, 20);
    // _ = _worksheet.write_string_with_format(0, count as u16, "备注", &Format::new().set_bold());
    _ = _worksheet.set_column_width(index, 26);
    let mut cindex = 1;
    // let color_format = Format::new().set_background_color(XlsxColor::RGB(0xf08409));
    for v in m {
        // println!("yd----{}", k.clone().unwrap());
        // ids.push_str(&format!("'{}',", v.shipment_code.clone().unwrap()));
        let aflag_text = aflag(v.audit_flag.unwrap());
        _ = _worksheet.write_string(cindex, 0, v.shipment_code.unwrap_or(String::default()));
        _ = _worksheet.write_string(cindex, 1, aflag_text);
        _ = _worksheet.write_string(cindex, 2, v.audit_reason.unwrap_or(String::default()));
        // _ = _worksheet.write_string(cindex, 3, v.audit_time.unwrap_or(String::default()));

        // let bold_italic_format = Format::new().set_bold().set_italic();
        // _worksheet.write_number_with_format(row, col, 232.21, &bold_italic_format);
        cindex += 1;
    }

    // println!("{}", path);

    workbook
        .save(format!("{}.xlsx", common::get_time_str()))
        .expect("保存Excel错误");
}
fn aflag(f: String) -> &'static str {
    let result = match f.as_str() {
        "1" => "初审通过",
        "2" => "初审驳回",
        _ => "未知状态",
    };
    result
}
#[derive(Serialize, Deserialize)]
pub struct InvoiceDetailsRoot {
    pub success: Option<bool>,
    pub message: Option<String>,
    pub code: Option<i64>,
    pub result: Option<InvoiceDetailsResult>,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceDetailsResult {
    pub records: Option<Vec<InvoiceDetailsRecord>>,
    pub total: Option<i64>,
    pub size: Option<i64>,
    pub current: Option<i64>,
    pub orders: Option<Vec<Option<serde_json::Value>>>,
    pub optimize_count_sql: Option<bool>,
    pub search_count: Option<bool>,
    pub count_id: Option<serde_json::Value>,
    pub max_limit: Option<serde_json::Value>,
    pub pages: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceDetailsRecord {
    pub id: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<String>,
    pub update_by: Option<String>,
    pub update_time: Option<String>,
    pub enterprise_id: Option<String>,
    pub fpsqh: Option<String>,
    pub ydsl: Option<i64>,
    pub fpsq_status: Option<String>,
    pub fphm: Option<serde_json::Value>,
    pub fpdm: Option<serde_json::Value>,
    pub fpys: Option<serde_json::Value>,
    pub xfnsrsbh: Option<String>,
    pub xfmc: Option<String>,
    pub gfnsrsbh: Option<String>,
    pub gfmc: Option<String>,
    pub gfdz: Option<String>,
    pub gfdh: Option<String>,
    pub gfkhh: Option<String>,
    pub gfzh: Option<String>,
    pub gfyhhb: Option<String>,
    pub shfnsrsbh: Option<String>,
    pub shfmc: Option<String>,
    pub fhfnsrsbh: Option<String>,
    pub fhfmc: Option<String>,
    pub gfhyg: Option<String>,
    pub fpnr: Option<serde_json::Value>,
    pub kpje: Option<String>,
    pub se: Option<String>,
    pub kpsl: Option<String>,
    pub kpzje: Option<String>,
    pub remark: Option<String>,
    pub fprq: Option<serde_json::Value>,
    pub xfdz: Option<String>,
    pub xfdh: Option<serde_json::Value>,
    pub xfkhh: Option<String>,
    pub xfzh: Option<String>,
    pub xfyhhb: Option<String>,
    pub xfcph: Option<String>,
    pub xfhyh: Option<String>,
    pub ydids: Option<String>,
    pub ydcodes: Option<serde_json::Value>,
    pub kpstatus: Option<i64>,
    pub tax_sync_date: Option<String>,
    pub audit_one: Option<String>,
    pub audit_two: Option<String>,
    pub audit_two_reason: Option<serde_json::Value>,
    pub audit_one_reason: Option<serde_json::Value>,
    pub ysqdhm: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
    pub wtdpdf: Option<String>,
    pub dksqje: Option<serde_json::Value>,
    pub sjkpje: Option<serde_json::Value>,
    pub ybsfe: Option<serde_json::Value>,
    pub is_down: Option<i64>,
    pub sqdhm: Option<String>,
    pub tj_time: Option<String>,
    pub audit_time: Option<String>,
    pub mark: Option<serde_json::Value>,
    pub mark_by: Option<serde_json::Value>,
    pub mark_time: Option<serde_json::Value>,
    #[serde(rename = "fpsqStatus_dictText")]
    pub fpsq_status_dict_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListByIdsRoot {
    pub success: Option<bool>,
    pub message: Option<String>,
    pub code: Option<i64>,
    pub result: Option<Vec<ListByIdsResult>>,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListByIdsResult {
    pub shipment_code: Option<String>,
    pub audit_reason: Option<String>,
    pub audit_flag: Option<String>,
}
