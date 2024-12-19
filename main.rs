use csv;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::{Mutex,Arc};
use std::{thread, time};
use std::cmp;


lazy_static! {
    static ref rate:Arc<Mutex<HashMap<String,f64>>> =  Arc::new(Mutex::new(HashMap::new()));
    static ref profit:Arc<Mutex<HashMap<String,f64>>> =  Arc::new(Mutex::new(HashMap::new()));
    static ref avgs:Arc<Mutex<HashMap<String,f64>>> =  Arc::new(Mutex::new(HashMap::new()));

}
fn read(path:&str) -> Vec<f64>{
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let data:Vec<f64> = rdr.records().map(|r| r.unwrap().get(0).unwrap().parse::<f64>().unwrap()).collect();

    data
}

fn main() {
    let shares: [&str; 158] = ["DATA","IRKT","VSMO","UNAC","VKCO","TTLK","MGNT","SPBE","SVCB"
        ,"ETLN","KZOSP","WUSH","GEMC","UGLD","PHOR","HNFG","HHRU","LNZL"
        ,"SELG","TATNP","PRFN","MAGN","VTBR","CARM","RUAL","NKHP"
        ,"BANEP","OKEY","ALRS","ELMT","MRKP","FLOT","DIAS","TATN","ABIO","UWGN"
        ,"DVEC","RTKM","PRMD","ZAYM","MTSS","OZPH","TGKN","TRNFP","FEES"
        ,"PMSB","MSRS","IRAO","NSVZ","GCHE","SNGSP","NVTK","UNKL","NKNC"
        ,"AQUA","VRSB","MBNK","MOEX","ROLO","OZON","OGKB","GLTR","KAZTP"
        ,"SNGS","CBOM","AMEZ","TGKBP","ABRD","PIKK","ROSN","EUTR","TRMK","SOFL"
        ,"MRKU","BLNG","KRKNP","CHMF","ENPG","NKNCP","T","MRKV"
        ,"LSRG","CNTL","SFTL","VSEH","SVAV","RTKMP","KMAZ","KZOS","FIXP","MGTSP"
        ,"ELFV","KLVZ","VEON-RX","HEAD","YDEX","SMLT","BANE","AFLT","CIAN"
        ,"ORUP","SBER","GECO","MVID","PMSBP","MSTT","MTLR","IVAT","AKRN","MDMG"
        ,"GAZP","SBERP","LENT","BSPB","ASTR","RKKE","LSNG","RENI","MRKC","POSI"
        ,"KROT","KAZT","SIBN","YAKG","TGKB","KLSB","RBCM","CHMK","RNFT","MRKZ"
        ,"APTK","QIWI","HYDR","GTRK","NLMK","BELU","LNZLP","SFIN","FESH","RASP"
        ,"SGZH","LSNGP","GMKN","LIFE","UPRO","MRKS","MSNG","AFKS","PLZL"
        ,"LKOH","DELI","MGKL","LEAS","NMTP","TGKA","CNTLP","MTLRP","MRKY"];

    let mut handles = vec![];

    let mut rates = rate.lock().unwrap();
    rates.insert(String::from("SUMM"), 0.0);
    rates.insert(String::from("COEFF"), 0.0);
    rates.insert(String::from("C1B"), 1.0);
    rates.insert(String::from("C2B"), 1.0);
    rates.insert(String::from("C1S"), 1.0);
    rates.insert(String::from("C2S"), 1.0);
    rates.insert(String::from("LOSS"), 1.0);
    rates.insert(String::from("GROSS"), 1.0);
    rates.insert(String::from("AVG_PROFIT"), 1.0);
    drop(rates);

    let mut profits = profit.lock().unwrap();
    profits.insert(String::from("SUMM"), 0.0);
    profits.insert(String::from("COEFF"), 0.0);
    profits.insert(String::from("C1B"), 1.0);
    profits.insert(String::from("C2B"), 1.0);
    profits.insert(String::from("C1S"), 1.0);
    profits.insert(String::from("C2S"), 1.0);
    profits.insert(String::from("LOSS"), 1.0);
    profits.insert(String::from("GROSS"), 1.0);
    profits.insert(String::from("AVG_PROFIT"), 1.0);
    drop(profits);

    let mut avg = avgs.lock().unwrap();
    avg.insert(String::from("SUMM"), 0.0);
    avg.insert(String::from("COEFF"), 0.0);
    avg.insert(String::from("C1B"), 1.0);
    avg.insert(String::from("C2B"), 1.0);
    avg.insert(String::from("C1S"), 1.0);
    avg.insert(String::from("C2S"), 1.0);
    avg.insert(String::from("LOSS"), 1.0);
    avg.insert(String::from("GROSS"), 1.0);
    avg.insert(String::from("AVG_PROFIT"), 1.0);
    drop(avg);


    for first in 1..50 {
        let thread_ = thread::spawn(move || {
            for second in 1..50 {
                for first_sell in 1..50 {
                    for second_sell in 1..50 {
                        if first_sell != second_sell && first != second{
                            let mut loss = 1.0;
                            let mut gross = 1.0;
                            let mut summ = 0.0;
                            let mut avg_profit = vec![];

                            for ticker in shares.iter() {

                                let mut enter_price = 0.0;
                                let mut qty = 0.0;
                                let mut in_position = false;
                                let mut cash = 20000.0;

                                let path = format!("D:/back/csv/{ticker}.csv");
                                let data = read(path.as_str());

                                for (index,element) in data.iter().enumerate() {
                                    if in_position == false {
                                        if index >= cmp::max(first,second){
                                            let ema_fast = &data[index - (first - 1)..=index].into_iter().sum::<f64>() / first as f64;
                                            let ema_mid = &data[index - (second - 1)..=index].into_iter().sum::<f64>() / second as f64;
                                            let ema_fast_last = &data[index - first..index].into_iter().sum::<f64>() / first as f64;
                                            let ema_mid_last = &data[index - second..index].into_iter().sum::<f64>() / second as f64;

                                            if ema_fast < ema_mid && ema_mid_last <= ema_fast_last {

                                                if cash / element >= 1.0{
                                                    in_position = true;
                                                    qty = cash / element;
                                                    cash = cash - (element * qty);
                                                    enter_price = *element;
                                                }
                                            }
                                        }
                                    }

                                    else {
                                        if index >= cmp::max(first_sell,second_sell){
                                            let ema_fast_sell = &data[index - (first_sell - 1)..=index].into_iter().sum::<f64>() / first_sell as f64;
                                            let ema_mid_sell = &data[index - (second_sell - 1)..=index].into_iter().sum::<f64>() / second_sell as f64;

                                            let ema_fast_last_sell = &data[index - first_sell..index].into_iter().sum::<f64>() / first_sell as f64;
                                            let ema_mid_last_sell = &data[index - second_sell..index].into_iter().sum::<f64>() / second_sell as f64;
                                            if ema_fast_sell > ema_mid_sell && ema_mid_last_sell >= ema_fast_last_sell {

                                                cash = cash + (element * qty);
                                                avg_profit.push((element * qty) - (enter_price * qty));

                                                in_position = false;
                                                if enter_price <= *element{
                                                    gross += 1.0;
                                                }
                                                else {
                                                    loss += 1.0;
                                                }
                                            }
                                        }
                                    }
                                }
                                if in_position{
                                    cash = cash + qty * data[data.len() - 1];

                                    in_position = false;
                                    if enter_price <= data[data.len() -1]{
                                        gross += 1.0;
                                    }
                                    else {
                                        loss += 1.0;
                                    }
                                }

                                summ += cash - 20000.0;
                            }


                            let mut rates = Arc::clone(&*rate);
                            if rates.lock().unwrap().get("COEFF") < (&(gross / loss)).into() {
                                rates.lock().unwrap().insert("SUMM".to_string(),summ);
                                rates.lock().unwrap().insert("COEFF".to_string(),gross / loss);
                                rates.lock().unwrap().insert("C1B".to_string(),first as f64);
                                rates.lock().unwrap().insert("C2B".to_string(),second as f64);
                                rates.lock().unwrap().insert("C1S".to_string(),first_sell as f64);
                                rates.lock().unwrap().insert("C2S".to_string(), second_sell as f64);
                                rates.lock().unwrap().insert("LOSS".to_string(),loss);
                                rates.lock().unwrap().insert("GROSS".to_string(),gross);
                                rates.lock().unwrap().insert("AVG_PROFIT".to_string(),avg_profit.clone().into_iter().sum::<f64>() / avg_profit.len() as f64);

                            }

                            let mut profits = Arc::clone(&*profit);
                            if *profits.lock().unwrap().get("SUMM").unwrap() < summ {
                                profits.lock().unwrap().insert("SUMM".to_string(),summ);
                                profits.lock().unwrap().insert("COEFF".to_string(),gross / loss);
                                profits.lock().unwrap().insert("C1B".to_string(),first as f64);
                                profits.lock().unwrap().insert("C2B".to_string(),second as f64);
                                profits.lock().unwrap().insert("C1S".to_string(),first_sell as f64);
                                profits.lock().unwrap().insert("C2S".to_string(), second_sell as f64);
                                profits.lock().unwrap().insert("LOSS".to_string(),loss);
                                profits.lock().unwrap().insert("GROSS".to_string(),gross);
                                profits.lock().unwrap().insert("AVG_PROFIT".to_string(),avg_profit.clone().into_iter().sum::<f64>() / avg_profit.len() as f64);

                            }
                            let mut avg = Arc::clone(&*avgs);
                            if *avg.lock().unwrap().get("AVG_PROFIT").unwrap() < avg_profit.clone().into_iter().sum::<f64>() / avg_profit.len() as f64 {
                                avg.lock().unwrap().insert("SUMM".to_string(),summ);
                                avg.lock().unwrap().insert("COEFF".to_string(),gross / loss);
                                avg.lock().unwrap().insert("C1B".to_string(),first as f64);
                                avg.lock().unwrap().insert("C2B".to_string(),second as f64);
                                avg.lock().unwrap().insert("C1S".to_string(),first_sell as f64);
                                avg.lock().unwrap().insert("C2S".to_string(), second_sell as f64);
                                avg.lock().unwrap().insert("LOSS".to_string(),loss);
                                avg.lock().unwrap().insert("GROSS".to_string(),gross);
                                avg.lock().unwrap().insert("AVG_PROFIT".to_string(),avg_profit.clone().into_iter().sum::<f64>() / avg_profit.len() as f64);

                            }
                            drop(avg);
                            drop(profits);
                            drop(rates);

                            if first == 1 {
                                let mut rates = Arc::clone(&*rate);
                                let mut profits = Arc::clone(&*profit);
                                let mut avg = Arc::clone(&*avgs);

                                println!("BUY [{},{}] SELL [{},{}]",first,second,first_sell,second_sell);
                                println!(" ");
                                println!("RATES>>>>>>>>>>>>>");
                                println!("SUMM {:?}",rates.lock().unwrap().get("SUMM").unwrap());
                                println!("COEFF {:?}",rates.lock().unwrap().get("COEFF").unwrap());
                                println!("LOSS {:?}",rates.lock().unwrap().get("LOSS").unwrap());
                                println!("GROSS {:?}",rates.lock().unwrap().get("GROSS").unwrap());
                                println!("C1B {:?}",rates.lock().unwrap().get("C1B").unwrap());
                                println!("C2B {:?}",rates.lock().unwrap().get("C2B").unwrap());
                                println!("C1S {:?}",rates.lock().unwrap().get("C1S").unwrap());
                                println!("C2S {:?}",rates.lock().unwrap().get("C2S").unwrap());
                                println!("AVG_PROFIT {:?}",rates.lock().unwrap().get("AVG_PROFIT").unwrap());
                                println!(" ");
                                println!("PROFIT>>>>>>>>>>>>");
                                println!("SUMM {:?}",profits.lock().unwrap().get("SUMM").unwrap());
                                println!("COEFF {:?}",profits.lock().unwrap().get("COEFF").unwrap());
                                println!("LOSS {:?}",profits.lock().unwrap().get("LOSS").unwrap());
                                println!("GROSS {:?}",profits.lock().unwrap().get("GROSS").unwrap());
                                println!("C1B {:?}",profits.lock().unwrap().get("C1B").unwrap());
                                println!("C2B {:?}",profits.lock().unwrap().get("C2B").unwrap());
                                println!("C1S {:?}",profits.lock().unwrap().get("C1S").unwrap());
                                println!("C2S {:?}",profits.lock().unwrap().get("C2S").unwrap());
                                println!("AVG_PROFIT {:?}",profits.lock().unwrap().get("AVG_PROFIT").unwrap());
                                println!(" ");
                                println!("AVG>>>>>>>>>>>>>");
                                println!("SUMM {:?}",avg.lock().unwrap().get("SUMM").unwrap());
                                println!("COEFF {:?}",avg.lock().unwrap().get("COEFF").unwrap());
                                println!("LOSS {:?}",avg.lock().unwrap().get("LOSS").unwrap());
                                println!("GROSS {:?}",avg.lock().unwrap().get("GROSS").unwrap());
                                println!("C1B {:?}",avg.lock().unwrap().get("C1B").unwrap());
                                println!("C2B {:?}",avg.lock().unwrap().get("C2B").unwrap());
                                println!("C1S {:?}",avg.lock().unwrap().get("C1S").unwrap());
                                println!("C2S {:?}",avg.lock().unwrap().get("C2S").unwrap());
                                println!("AVG_PROFIT {:?}",avg.lock().unwrap().get("AVG_PROFIT").unwrap());

                                drop(rates);
                                drop(profits);
                                drop(avg);

                            }
                        }
                    }
                }
            }
        });
        handles.push(thread_);

    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("End");
}
