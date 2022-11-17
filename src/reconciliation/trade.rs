use anyhow::anyhow;
use std::str::FromStr;

// 交易
#[derive(Debug, PartialEq, Eq)]
pub struct Trade {
    // 序号
    pub id: String,
    // 记账标志
    pub booking_flag: String,
    // 转出交易网会员代码
    pub out_tran_net_member_code: String,
    // 转出子账户
    pub out_sub_account_no: String,
    // 转出子账户名称
    pub out_sub_account_name: String,
    // 转入交易网会员代码
    pub in_tran_net_member_code: String,
    // 转入子账户
    pub in_sub_account_no: String,
    // 转入子账户名称
    pub in_sub_account_name: String,
    // 交易金额
    pub tran_amount: String,
    // 手续费
    pub commission: String,
    // 交易日期
    pub tran_date: String,
    // 交易时间
    pub tran_time: String,
    // 银行见证系统流水号
    pub front_seq_no: String,
    // 交易网流水号
    pub cnsmr_seq_no: String,
    // 备注
    pub remark: String,
    // 订单号
    pub order_no: String,
}

// 5&确认付款&246046509282361345&3492000000001178&福建xxx人力资源股份有限公司&100085&3492000000001068&陈xx&1.00&0.01&20221004&000004&2210044947980889&W306892210041424305561&&427191907240640513&
impl FromStr for Trade {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vs = s.split("&");

        let id = vs.next().ok_or(anyhow!("未找到序号"))?.to_owned();
        let booking_flag = vs.next().ok_or(anyhow!("未找到记账标志"))?.to_owned();
        let out_tran_net_member_code = vs
            .next()
            .ok_or(anyhow!("未找到转出交易网会员代码"))?
            .to_owned();
        let out_sub_account_no = vs.next().ok_or(anyhow!("未找到转出子账号"))?.to_owned();
        let out_sub_account_name = vs.next().ok_or(anyhow!("未找到转出子账号名称"))?.to_owned();
        let in_tran_net_member_code = vs
            .next()
            .ok_or(anyhow!("未找到转入交易网会员代码"))?
            .to_owned();
        let in_sub_account_no = vs.next().ok_or(anyhow!("未找到转入子账号"))?.to_owned();
        let in_sub_account_name = vs.next().ok_or(anyhow!("未找到转入子账号名称"))?.to_owned();
        let tran_amount = vs.next().ok_or(anyhow!("未找到交易金额"))?.to_owned();
        let commission = vs.next().ok_or(anyhow!("未找到手续费"))?.to_owned();
        let tran_date = vs.next().ok_or(anyhow!("未找到交易日期"))?.to_owned();
        let tran_time = vs.next().ok_or(anyhow!("未找到交易时间"))?.to_owned();
        let front_seq_no = vs.next().ok_or(anyhow!("未找到银行见证流水号"))?.to_owned();
        let cnsmr_seq_no = vs.next().ok_or(anyhow!("未找到交易网流水号"))?.to_owned();
        let remark = vs.next().ok_or(anyhow!("未找到备注"))?.to_owned();
        let order_no = vs.next().ok_or(anyhow!("未找到订单号"))?.to_owned();

        Ok(Self {
            id,
            booking_flag,
            out_tran_net_member_code,
            out_sub_account_no,
            out_sub_account_name,
            in_tran_net_member_code,
            in_sub_account_no,
            in_sub_account_name,
            tran_amount,
            commission,
            tran_date,
            tran_time,
            front_seq_no,
            cnsmr_seq_no,
            remark,
            order_no,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Trade;

    #[test]
    fn parse_string_to_trade() -> Result<(), anyhow::Error> {
        let s = "5&确认付款&246046509282361345&3492000000001178&福建xxx人力资源股份有限公司&100085&3492000000001068&陈xx&1.00&0.01&20221004&000004&2210044947980889&W306892210041424305561&&427191907240640513&";
        let v: Trade = s.parse()?;
        assert_eq!(
            v,
            Trade {
                id: "5".to_owned(),
                booking_flag: "确认付款".to_owned(),
                out_tran_net_member_code: "246046509282361345".to_owned(),
                out_sub_account_no: "3492000000001178".to_owned(),
                out_sub_account_name: "福建xxx人力资源股份有限公司".to_owned(),
                in_tran_net_member_code: "100085".to_owned(),
                in_sub_account_no: "3492000000001068".to_owned(),
                in_sub_account_name: "陈xx".to_owned(),
                tran_amount: "1.00".to_owned(),
                commission: "0.01".to_owned(),
                tran_date: "20221004".to_owned(),
                tran_time: "000004".to_owned(),
                front_seq_no: "2210044947980889".to_owned(),
                cnsmr_seq_no: "W306892210041424305561".to_owned(),
                remark: "".to_owned(),
                order_no: "427191907240640513".to_owned(),
            }
        );
        Ok(())
    }
}
