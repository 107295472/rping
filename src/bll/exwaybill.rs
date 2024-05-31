use std::time::Duration;

use crate::commons::{common, consts, get_config::CFG, model::AlctAPIModel};
use redis::Commands;
use reqwest::header;
use rust_xlsxwriter::{Format, Workbook};
use serde::{Deserialize, Serialize};
pub async fn ex_waybill_gen() {
    // let mut hashmap: HashMap<Option<String>, Option<String>> = HashMap::new();
    let mut result: Vec<Record> = Vec::new();
    for n in 0..17 {
        let mut con = common::get_redis().unwrap();
        let dd = con.get(consts::TOKEN);
        let para=format!("freight-tax/complain/anfComplain/ssList?_t={}&column=createTime&order=desc&field=id,,kpStatus,auditFlag,shipmentCode,auditReason,action&pageNo={}&pageSize=30",common::get_timestamp(true),n+1);
        match dd {
            Ok(token) => {
                // dbg!(t);
                // let t: LocalToken = serde_json::from_str(&token).unwrap();
                let req = AlctAPIModel {
                    menthod: "get".to_string(),
                    token: Some(token),
                    method_adress: para,
                    method_domain: CFG.system.method_domain.clone(),
                    data: String::default(),
                };
                let resdata = send_api(req).await.unwrap();
                // let ress = match resdata {
                //     Ok(v) => v,
                //     Err(e) => panic!("{}", e.to_string()),
                // };
                let res: RootNode = serde_json::from_str(&resdata).unwrap();
                // let db = Database::connect("sqlite::memory:")
                //     .await
                //     .expect("数据库初始错误");
                // println!("code:{},for index:{}", res.code.unwrap(), n);
                let records: Option<Vec<Record>> = res.result.unwrap().records;
                // println!("{},{},{}", n, records.is_none(), records.unwrap().len());
                for i in records.unwrap().into_iter() {
                    // println!("{}", i.kp_status.clone().unwrap());
                    // hashmap.insert(i.shipment_code.clone(), i.kp_status.clone());
                    result.push(i);
                }
                // println!("len原始---{}", hashmap.len());

                // for (contact, number) in hashmap.iter() {
                //     println!(
                //         "data {}: {}",
                //         contact.clone().unwrap(),
                //         number.clone().unwrap()
                //     );
                // }
            }
            Err(e) => panic!("token不存在:{}", e),
        };
        common::sleep_async(2).await;
    }
    gen_excel(result).await;
}
async fn gen_excel(m: Vec<Record>) {
    println!("len---{}", m.len());
    let mut workbook = Workbook::new();

    let _worksheet = workbook.add_worksheet();
    _ = _worksheet.set_name("Sheet0");
    // global::info_msg("2222222222");
    let colv = vec![
        "发票申请号",
        "运单编号",
        "缺少车头照片",
        "缺少车身照片",
        "缺少磅单",
        "无问题",
    ];
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
        _ = _worksheet.write_string(cindex, 0, v.kp_status.unwrap_or(String::default()));
        _ = _worksheet.write_string(cindex, 1, v.shipment_code.unwrap_or(String::default()));
        _ = _worksheet.write_string(cindex, 2, &String::default());
        _ = _worksheet.write_string(cindex, 3, &String::default());
        _ = _worksheet.write_string(cindex, 4, &String::default());

        // let bold_italic_format = Format::new().set_bold().set_italic();
        // _worksheet.write_number_with_format(row, col, 232.21, &bold_italic_format);
        cindex += 1;
    }

    // println!("{}", path);
    workbook
        .save(format!("{}.xlsx", common::get_time_str()))
        .expect("保存Excel错误");
}
pub async fn send_api(m: AlctAPIModel) -> reqwest::Result<String> {
    // let mut data = String::from("");
    // data = serde_json::to_string(m.data);
    let mut headers = header::HeaderMap::new();
    // global::info_msg(&m.token.clone().unwrap());
    if m.token.is_some() {
        let auth_value = header::HeaderValue::from_str(&m.token.unwrap()).unwrap();
        // auth_value.set_sensitive(true);

        headers.insert("X-Access-Token", auth_value);
        let timestamp_value: header::HeaderValue =
            header::HeaderValue::from_str(&common::get_time_str()).unwrap();
        headers.insert("X-Timestamp", timestamp_value);
    }
    // global::info("mark", m.token.unwrap());
    let t = header::HeaderValue::from_str("application/json").unwrap();
    headers.insert(header::CONTENT_TYPE, t);
    // headers.insert(
    //     header::SET_COOKIE,
    //     header::HeaderValue::from_str("SECKEY_ABVK=j+EMcjBBTfKwcuMHE7hpvEAEFVcoa1fCT8lvZCzvX4c%3D")
    //         .unwrap(),
    // );
    // headers.insert(
    //     header::SET_COOKIE,
    //     header::HeaderValue::from_str("BMAP_SECKEY=RCWUmamYqZD9CXqp5Ci9FO8PO7cUaHGu2FN2c9l-gZHErZWlsKZJ27SHK2btmgriwB4A8eg46YKlDXipFTideJ0BFqyRy6paRaL715RDkcos_HhzWtjlSdZbgcA7S8mF2BRMYH0kZjb_NPzg5y_fXTsidf3rbly9IeCKVlAkh6kB5qG4Or1BouGOkpSSKVKr")
    //         .unwrap(),
    // );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(50000))
        .build()
        .unwrap();

    let url = format!("{}{}", m.method_domain, m.method_adress);
    // global::info("post", url.clone());
    if m.menthod == "post" {
        let res = client.post(url).body(m.data.clone()).send().await;
        match res {
            Ok(t) => t.text().await,
            Err(e) => {
                panic!("{},send:{}", e, m.data)
            }
        }
    } else if m.menthod == "put" {
        let res = client.put(url).body(m.data).send().await;
        match res {
            Ok(t) => t.text().await,

            Err(e) => {
                panic!("{}", e)
            }
        }
    } else {
        let res = client.get(url.clone()).body(m.data).send().await;
        // dbg!(url);
        match res {
            Ok(t) => t.text().await,

            Err(e) => {
                panic!("{}", e)
            }
        }
    }
}
// async fn setup_schema(db: &DbConn) {
//     // Setup Schema helper
//     let schema = Schema::new(DbBackend::Sqlite);

//     // Derive from Entity
//     let stmt: TableCreateStatement = schema.create_table_from_entity(MyEntity);

//     // Or setup manually
//     assert_eq!(
//         stmt.build(SqliteQueryBuilder),
//         Table::create()
//             .table(MyEntity)
//             .col(ColumnDef::new(MyEntity::Column::Id).integer().not_null())
//             //...
//             .build(SqliteQueryBuilder)
//     );

//     // Execute create table statement
//     let result = db.execute(db.get_database_backend().build(&stmt)).await;
// }
#[derive(Serialize, Deserialize)]
pub struct RootNode {
    pub success: Option<bool>,
    pub message: Option<String>,
    pub code: Option<i64>,
    pub result: Option<Result>,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub records: Option<Vec<Record>>,
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
pub struct Record {
    pub id: Option<String>,
    // pub create_by: Option<CreateBy>,
    // pub create_time: Option<String>,
    // pub update_by: Option<AuditOne>,
    // pub update_time: Option<String>,
    // pub enterprise_id: Option<String>,
    pub shipment_code: Option<String>,
    // pub shipping_note_number: Option<String>,
    // pub consignment_date_time: Option<String>,
    // pub business_type_code: Option<i64>,
    // pub pickup_location: Option<String>,
    // pub pickup_country_subdivision_code: Option<i64>,
    // pub pickup_lng: Option<f64>,
    // pub pickup_lat: Option<f64>,
    // pub unload_location: Option<String>,
    // pub unload_country_subdivision_code: Option<i64>,
    // pub unload_lng: Option<f64>,
    // pub unload_lat: Option<f64>,
    // pub distance: Option<i64>,
    // pub plan_pickup_time_start: Option<String>,
    // pub plan_pickup_time_end: Option<String>,
    // pub plan_unload_time_start: Option<String>,
    // pub plan_unload_time_end: Option<String>,
    // pub cargo_type_classification_code: Option<i64>,
    // pub driver_identification: Option<String>,
    // pub vehicle_number: Option<String>,
    // pub total_weight: Option<f64>,
    // pub total_cube: Option<i64>,
    // pub status_code: Option<i64>,
    // pub status: Option<Status>,
    // pub first_pick_up_date: Option<String>,
    // pub last_arrivated_time: Option<String>,
    // pub last_submit_coordinate_time: Option<serde_json::Value>,
    // pub receipt_time: Option<String>,
    // pub signee_receive_time: Option<String>,
    // pub audit_one: Option<AuditOne>,
    // pub audit_flag: Option<String>,
    // pub audit_reason: Option<String>,
    pub kp_status: Option<String>,
    // pub signee_error: Option<serde_json::Value>,
    // pub receipt_no: Option<String>,
    // pub receipt_error: Option<serde_json::Value>,
    // pub distance_baidu: Option<f64>,
    // pub transport_contract_no: Option<String>,
    // pub transport_contract_price: Option<i64>,
    // pub iscomplain: Option<String>,
    // pub valid: Option<Valid>,
    // pub trailer_vehicle_number: Option<serde_json::Value>,
    // pub signee: Option<serde_json::Value>,
    // pub signee_remarks: Option<serde_json::Value>,
    // pub receipt_by: Option<serde_json::Value>,
    // pub receipt_remarks: Option<serde_json::Value>,
    // pub tj_time: Option<String>,
    // pub audit_time: Option<String>,
    // pub js_time: Option<String>,
    // pub is_ios: Option<String>,
    // pub is_ycsj: Option<String>,
    // pub gfmc: Option<serde_json::Value>,
    // pub xfmc: Option<serde_json::Value>,
    // pub xfhyh: Option<serde_json::Value>,
    // pub price: Option<serde_json::Value>,
    // pub is_query: Option<serde_json::Value>,
    // #[serde(rename = "tjTime_begin")]
    // pub tj_time_begin: Option<serde_json::Value>,
    // #[serde(rename = "tjTime_end")]
    // pub tj_time_end: Option<serde_json::Value>,
    // #[serde(rename = "auditTime_begin")]
    // pub audit_time_begin: Option<serde_json::Value>,
    // #[serde(rename = "auditTime_end")]
    // pub audit_time_end: Option<serde_json::Value>,
    // pub page_size: Option<i64>,
    // pub page_current: Option<i64>,
    // #[serde(rename = "cargoTypeClassificationCode_dictText")]
    // pub cargo_type_classification_code_dict_text: Option<CargoTypeClassificationCodeDictText>,
    // #[serde(rename = "statusCode_dictText")]
    // pub status_code_dict_text: Option<Status>,
    // #[serde(rename = "auditFlag_dictText")]
    // pub audit_flag_dict_text: Option<AuditFlagDictText>,
}

#[derive(Serialize, Deserialize)]
pub enum AuditFlagDictText {
    #[serde(rename = "审核驳回")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum AuditOne {
    #[serde(rename = "马洪娇")]
    AuditOne,
    #[serde(rename = "王珊珊")]
    Empty,
    #[serde(rename = "赵紫凌")]
    Purple,
}

#[derive(Serialize, Deserialize)]
pub enum CargoTypeClassificationCodeDictText {
    #[serde(rename = "普货")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum CreateBy {
    L0823005,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "已结算")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum Valid {
    Y,
}
