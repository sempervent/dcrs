from sqlalchemy import Boolean, Column, ForeignKey, Integer, String
from sqlalchemy.orm import relationship

from ..database import Base


class ReviewModel(Base):
    __tablename__ = "reviews"
    id = Column(Integer, primary_key=True, index=True)
    content_id = Column(Integer, ForeignKey("contents.id"))
    reviewer_id = Column(Integer, ForeignKey("users.id"))
    approved = Column(Boolean, default=False)

    content = relationship("ContentModel", back_populates="reviews")
    reviewer = relationship("UserModel", back_populates="reviews")


class ContentModel(Base):
    __tablename__ = "contents"
    id = Column(Integer, primary_key=True, index=True)
    title = Column(String, index=True)
    creator_id = Column(Integer, ForeignKey("users.id"))

    reviews = relationship("ReviewModel", back_populates="content")
