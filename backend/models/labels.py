from sqlalchemy import Column, Integer, String, ForeignKey, Table
from sqlalchemy.orm import relationship
from ..database import Base

# Association table between content and labels.py
content_label_association = Table(
    'content_label_association',
    Base.metadata,
    Column('content_id', Integer, ForeignKey('contents.id')),
    Column('label_id', Integer, ForeignKey('labels.py.id'))
)

class LabelModel(Base):
    __tablename__ = "labels.py"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)

    contents = relationship("ContentModel", secondary=content_label_association, back_populates="labels.py")
