from typing import Dict, List

from pydantic import BaseModel


class Review(BaseModel):
    content_id: int
    reviewer_id: int
    approved: bool
