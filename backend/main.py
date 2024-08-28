from fastapi import FastAPI, File, Form, UploadFile
from fastapi.middleware.cors import CORSMiddleware

from models.content import ContentModel
from models.labels import

app = FastAPI()

# Allow CORS for frontend requests
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/api/posts/trending")
async def get_trending_posts():
    # Replace with logic to fetch trending posts
    return [{"id": 1, "title": "Trending Post 1", "image": "/path/to/image1.jpg"}]


@app.get("/api/posts/recent")
async def get_recent_posts():
    # Replace with logic to fetch recent posts
    return [{"id": 2, "title": "Recent Post 1", "image": "/path/to/image2.jpg"}]


@app.get("/api/posts/user")
async def get_user_posts():
    # Replace with logic to fetch posts by the current user
    return [{"id": 3, "title": "Your Post 1", "image": "/path/to/image3.jpg"}]


@app.post("/api/posts")
async def create_post(content: str = Form(...), files: List[UploadFile] = File(...)):
    # Logic to handle the content and files
    new_content = ContentModel(title=content[:50], creator_id=current_user.id)  # Example: Creating a content entry
    db.add(new_content)
    db.commit()
    db.refresh(new_content)

    # Handle file uploads here
    for file in files:
        file_path = f"./uploads/{file.filename}"
        with open(file_path, "wb") as f:
            f.write(file.file.read())

    return {"id": new_content.id, "message": "Post created successfully"}
