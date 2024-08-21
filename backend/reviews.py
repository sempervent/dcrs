from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session

from .database import get_db
from .models import ContentModel, ReviewModel, UserModel
from .schemas import Review

router = APIRouter()


@router.post("/reviews/")
async def submit_review(
    review: Review,
    db: Session = Depends(get_db),
    current_user: UserModel = Depends(get_current_user),
):
    content = (
        db.query(ContentModel).filter(ContentModel.id == review.content_id).first()
    )

    if content.creator_id == current_user.id:
        raise HTTPException(
            status_code=400, detail="You cannot review your own content."
        )

    existing_review = (
        db.query(ReviewModel)
        .filter(
            ReviewModel.content_id == review.content_id,
            ReviewModel.reviewer_id == current_user.id,
        )
        .first()
    )

    if existing_review:
        raise HTTPException(
            status_code=400, detail="You have already reviewed this content."
        )

    new_review = ReviewModel(
        content_id=review.content_id,
        reviewer_id=current_user.id,
        approved=review.approved,
    )
    db.add(new_review)
    db.commit()
    db.refresh(new_review)

    return new_review


@router.get("/reviews/{content_id}/status")
async def check_review_status(content_id: int, db: Session = Depends(get_db)):
    reviews = db.query(ReviewModel).filter(ReviewModel.content_id == content_id).all()

    approvals = sum(1 for review in reviews if review.approved)
    if approvals >= 3:
        return {"status": "approved"}
    return {"status": "pending"}


@router.get("/reviews/pending")
async def get_pending_content(
    db: Session = Depends(get_db), current_user: UserModel = Depends(get_current_user)
):
    # Get content that the current user hasn't reviewed and that needs review
    pending_content = (
        db.query(ContentModel)
        .outerjoin(ReviewModel)
        .filter(
            ReviewModel.reviewer_id != current_user.id,
            ReviewModel.content_id.is_(None),  # Content with no reviews
        )
        .all()
    )

    return pending_content
