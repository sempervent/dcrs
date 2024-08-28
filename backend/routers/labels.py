from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session
from .models import LabelModel, ContentModel, UserModel
from .database import get_db
from .schemas import Label

router = APIRouter()

@router.get("/labels/")
async def get_labels(db: Session = Depends(get_db)):
    labels = db.query(LabelModel).all()
    return labels

@router.post("/labels/")
async def create_label(label: Label, db: Session = Depends(get_db), current_user: UserModel = Depends(get_current_user)):
    existing_label = db.query(LabelModel).filter(LabelModel.name == label.name).first()
    if existing_label:
        return existing_label

    new_label = LabelModel(name=label.name)
    db.add(new_label)
    db.commit()
    db.refresh(new_label)
    return new_label

@router.post("/contents/{content_id}/labels/")
async def add_labels_to_content(content_id: int, labels: List[str], db: Session = Depends(get_db), current_user: UserModel = Depends(get_current_user)):
    content = db.query(ContentModel).filter(ContentModel.id == content_id, ContentModel.creator_id == current_user.id).first()

    if not content:
        raise HTTPException(status_code=404, detail="Content not found or you are not the creator.")

    label_objects = []
    for label_name in labels:
        label = db.query(LabelModel).filter(LabelModel.name == label_name).first()
        if not label:
            label = LabelModel(name=label_name)
            db.add(label)
            db.commit()
            db.refresh(label)
        label_objects.append(label)

    content.labels.extend(label_objects)
    db.commit()

    return content
