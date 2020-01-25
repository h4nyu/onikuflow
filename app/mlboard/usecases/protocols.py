import typing as t
from uuid import UUID
from datetime import datetime
from mlboard.models.protocols import IPoint, ITrace, IWorkspace


class ITraceUsecase(t.Protocol):
    async def all(self) -> t.Sequence[ITrace]: ...
    async def delete_by(self, id: UUID) -> None: ...
    async def register(self, name: str, workspace_id: UUID) -> UUID: ...


class IPointUsecase(t.Protocol):
    async def add_scalar(self, trace_id: UUID, value: float, ts: t.Optional[datetime] = None) -> None: ...
    async def add_scalars(self, values: t.Dict[UUID, float], ts: t.Optional[datetime] = None) -> None: ...
    async def range_by(self, trace_id: UUID, from_date: datetime, to_date: datetime) -> t.Sequence[IPoint]: ...
    async def filter_by_limit(self, trace_id: UUID, limit: int) -> t.Sequence[IPoint]: ...


class IWorkspaceUsecase(t.Protocol):
    async def all(self) -> t.Sequence[IWorkspace]: ...
    async def delete_by(self, id: UUID) -> None: ...
    async def register(self, name: str, params: t.Dict[str, t.Any]) -> UUID: ...
