import typing as t
from cytoolz.curried import pipe, map, partition_all
import pytest
import asyncio
from datetime import datetime
from mlboard.queries.point import PointQuery
from mlboard.infra.db.connectors import ContextManager
from mlboard.models.point import Point
from uuid import uuid4
import time


@pytest.fixture(scope='function', autouse=True)
async def prepare() -> None:
    async with ContextManager() as conn:
        await PointQuery(conn).delete()


@pytest.mark.asyncio
async def test_performance_of_insert() -> None:
    ts = datetime.now()
    trace_id = uuid4()
    points = pipe(
        range(100000),
        map(lambda x: Point(
            value=0,
            ts=ts,
            trace_id=trace_id,
        )),
        list
    )

    start = time.time()
    async with ContextManager() as conn:
        await PointQuery(conn).bulk_insert(points)
    duration = time.time() - start
    print(duration)
    print(f'insert rate:{len(points)/duration}')
