from uuid import UUID
from typing_extensions import Protocol
from datetime import datetime


class IPoint(Protocol):
    value: float
    ts: datetime
    trace_id: UUID


class ITrace(Protocol):
    id: UUID
    tag: str
    created_at: datetime
    updated_at: datetime
