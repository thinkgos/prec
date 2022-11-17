use anyhow::anyhow;
use std::str::FromStr;

// 充值(和提现模型是一样的)
#[derive(Debug, PartialEq, Eq)]
pub struct Recharge {
    // 序号
    pub id: String,
    // 交易网会员代码
    pub tran_net_member_code: String,
    // 子账号
    pub sub_account_no: String,
    // 子账户名称
    pub sub_account_name: String,
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
    // 记账类型
    pub booking: String,
    // 订单号
    pub order_no: String,
}

impl FromStr for Recharge {
    type Err = anyhow::Error;

    // 充值: 1&351233741965754369&3492000000147328&福州xx软件有限公司&300.00&0.00&20221004&192842&2210044998349942&0121222210041120538763&服务费&绑卡匹配入金&&
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vs = s.split("&");

        let id = vs.next().ok_or(anyhow!("未找到序号"))?.to_owned();
        let tran_net_member_code = vs.next().ok_or(anyhow!("未找到交易网会员代码"))?.to_owned();
        let sub_account_no = vs.next().ok_or(anyhow!("未找到子账号"))?.to_owned();
        let sub_account_name = vs.next().ok_or(anyhow!("未找到子账号名称"))?.to_owned();
        let tran_amount = vs.next().ok_or(anyhow!("未找到交易金额"))?.to_owned();
        let commission = vs.next().ok_or(anyhow!("未找到手续费"))?.to_owned();
        let tran_date = vs.next().ok_or(anyhow!("未找到交易日期"))?.to_owned();
        let tran_time = vs.next().ok_or(anyhow!("未找到交易时间"))?.to_owned();
        let front_seq_no = vs.next().ok_or(anyhow!("未找到银行见证流水号"))?.to_owned();
        let cnsmr_seq_no = vs.next().ok_or(anyhow!("未找到交易网流水号"))?.to_owned();
        let remark = vs.next().ok_or(anyhow!("未找到备注"))?.to_owned();
        let booking = vs.next().ok_or(anyhow!("未找到记账类型"))?.to_owned();
        let order_no = vs.next().ok_or(anyhow!("未找到订单号"))?.to_owned();

        Ok(Self {
            id,
            tran_net_member_code,
            sub_account_no,
            sub_account_name,
            tran_amount,
            commission,
            tran_date,
            tran_time,
            front_seq_no,
            cnsmr_seq_no,
            remark,
            booking,
            order_no,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Recharge;

    #[test]
    fn parse_string_to_recharge() -> Result<(), anyhow::Error> {
        let s = "1&351233741965754369&3492000000147328&福州xx软件有限公司&300.00&0.00&20221004&192842&2210044998349942&0121222210041120538763&服务费&绑卡匹配入金&&";
        let v: Recharge = s.parse()?;
        assert_eq!(
            v,
            Recharge {
                id: "1".to_owned(),
                tran_net_member_code: "351233741965754369".to_owned(),
                sub_account_no: "3492000000147328".to_owned(),
                sub_account_name: "福州xx软件有限公司".to_owned(),
                tran_amount: "300.00".to_owned(),
                commission: "0.00".to_owned(),
                tran_date: "20221004".to_owned(),
                tran_time: "192842".to_owned(),
                front_seq_no: "2210044998349942".to_owned(),
                cnsmr_seq_no: "0121222210041120538763".to_owned(),
                remark: "服务费".to_owned(),
                booking: "绑卡匹配入金".to_owned(),
                order_no: "".to_owned(),
            }
        );
        Ok(())
    }
}
