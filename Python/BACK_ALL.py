from get_data import get_datas
import asyncio
import vectorbt as vbt
import pandas

shares = ["DATA","IRKT","VSMO","UNAC","VKCO","TTLK","MGNT","SPBE","SVCB"
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
        ,"LKOH","DELI","MGKL","LEAS","NMTP","TGKA","CNTLP","MTLRP","MRKY"]
summ = 0
for i in shares:
    print(i)
    data = pandas.read_csv(f"D:/back/csv/{i}.csv")
    if data is not None:
        vbt.settings.portfolio['init_cash'] = 20000
        fast_ma = vbt.MA.run(data['close'], 4, short_name='fast')
        slow_ma = vbt.MA.run(data['close'], 1, short_name='slow')
        fast_ma_ = vbt.MA.run(data['close'], 5, short_name='sdfsdf')
        slow_ma_ = vbt.MA.run(data['close'], 1, short_name='dsf')
        entries = fast_ma.ma_crossed_below(slow_ma)
        exits = fast_ma_.ma_crossed_above(slow_ma_)
        pf = vbt.Portfolio.from_signals(
                data['close'],entries,exits,fees=0)
        summ = summ +  pf.stats()['End Value'] - 20000
        print(pf.stats()['End Value'] - 20000)

print(summ)

