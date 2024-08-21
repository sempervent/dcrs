from sqlalchemy import Column, ForeignKey, Integer, String
from sqlalchemy.orm import relationship
from ..database import Base


class ContentModel(Base):
    __tablename__ = "contents"
    id = Column(Integer, primary_key=True, index=True)
    title = Column(String, index=True)
    creator_id = Column(Integer, ForeignKey("users.id"))

    reviews = relationship("ReviewModel", back_populates="content")
    labels = relationship("LabelModel", secondary=content_label_association, back_populates="contents")
