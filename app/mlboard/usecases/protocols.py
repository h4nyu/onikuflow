import typing as t
from uuid import UUID
from typing_extensions import Protocol
from datetime import datetime
from mlboard.models.protocols import IPoint, ITrace


class ITraceUsecase(Protocol):
    async def all(self) -> t.Sequence[ITrace]: ...
    async def delete_by(self, id: UUID) -> None: ...
    async def register(self, tag: str) -> UUID: ...


class IPointUsecase(Protocol):
    async def add_scalar(self, trace_id: UUID, value: float) -> None: ...
    async def range_by(self, trace_id: UUID, from_date: datetime, to_date: datetime) -> t.Sequence[IPoint]: ...
    async def filter_by_limit(self, trace_id: UUID, limit: int) -> t.Sequence[IPoint]: ...
