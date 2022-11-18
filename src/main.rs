use anyhow::Result;
use polars::prelude::*;

use prec::reconciliation::Transformer;
use prec::reconciliation::{Recharge, Surplus, Trade, Withdraw};

fn main() -> Result<()> {
    let recharge = vec![
        Recharge {
            id: "1".to_owned(),
            tran_net_member_code: "2".to_owned(),
            sub_account_no: "3".to_owned(),
            sub_account_name: "4".to_owned(),
            tran_amount: "5".to_owned(),
            commission: "6".to_owned(),
            tran_date: "7".to_owned(),
            tran_time: "8".to_owned(),
            front_seq_no: "9".to_owned(),
            cnsmr_seq_no: "a".to_owned(),
            remark: "b".to_owned(),
            booking: "c".to_owned(),
            order_no: "d".to_owned(),
        },
        Recharge {
            id: "11".to_owned(),
            tran_net_member_code: "12".to_owned(),
            sub_account_no: "13".to_owned(),
            sub_account_name: "14".to_owned(),
            tran_amount: "15".to_owned(),
            commission: "16".to_owned(),
            tran_date: "17".to_owned(),
            tran_time: "18".to_owned(),
            front_seq_no: "19".to_owned(),
            cnsmr_seq_no: "1a".to_owned(),
            remark: "1b".to_owned(),
            booking: "1c".to_owned(),
            order_no: "1d".to_owned(),
        },
    ];
    let df = recharge.transform()?;

    println!("{}", df);
    Ok(())
}
