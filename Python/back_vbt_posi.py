from get_data import get_datas
import asyncio
import vectorbt as vbt
import pandas



data = pandas.read_csv("D:/back/csv/VRSB.csv")
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
    print(pf.stats())
    pf.plot().show()
