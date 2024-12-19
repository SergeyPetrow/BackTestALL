import asyncio

import aiohttp

import aiomoex
import pandas as pd


async def get_datas(interval,ticker):
    try:
        async with aiohttp.ClientSession() as session:
            data = await aiomoex.get_board_candles(session,interval=interval,security=ticker)
            data = pd.DataFrame(data)

            datas = pd.DataFrame()
            datas['datetime'] = data['begin']
            datas['open'] = data['open']
            datas['close'] = data['close']
            datas['high'] = data['high']
            datas['volume'] = data['volume']

            datas['datetime'] = pd.to_datetime(datas['datetime'])
            datas.index = datas['datetime']

            return datas
    except KeyError:
        return None

