from faker import Faker
from pymongo import MongoClient
import os
import sys
from pathlib import Path

# โหลด .env โดยค้นหาพาเรนต์จนกว่าจะเจอ (robust when running from anywhere)
def load_project_env():
    try:
        from dotenv import load_dotenv
    except Exception:
        return None

    # start from this script dir, go up to repo root
    p = Path(__file__).resolve().parent
    for _ in range(10):
        candidate = p / ".env"
        if candidate.exists():
            load_dotenv(candidate)
            print(f"Loaded .env from: {candidate}")
            return str(candidate)
        p = p.parent
    # try find_dotenv as fallback
    try:
        from dotenv import find_dotenv
        f = find_dotenv(usecwd=True)
        if f:
            load_dotenv(f)
            print(f"Loaded .env via find_dotenv: {f}")
            return f
    except Exception:
        pass
    return None

load_project_env()

fake = Faker()

# CLI: python scripts/seed.py [count] [mongo_uri] [db_name]
count = int(sys.argv[1]) if len(sys.argv) > 1 else int(os.getenv("SEED_COUNT", "100000"))
uri = sys.argv[2] if len(sys.argv) > 2 else os.getenv("MONGODB_URI")
db_name = sys.argv[3] if len(sys.argv) > 3 else os.getenv("MONGODB_DB")

if not uri or not db_name:
    print("Error: MONGODB_URI or MONGODB_DB not set. Provide via .env, env vars or CLI args.")
    print("Usage: python scripts/seed.py [count] [mongodb_uri] [db_name]")
    sys.exit(1)

print(f"Seeding {count} documents into {db_name} at {uri}")

client = MongoClient(uri)
db = client[db_name]
books = []
for i in range(count):
    # fake.sentence may include punctuation; strip and add index to ensure uniqueness
    title = fake.sentence(nb_words=3).rstrip('.')
    try:
        year = int(fake.year())
    except Exception:
        year = 2000
    books.append({
        "name": f"{title}-{i}",
        "author": fake.name(),
        "year": year
    })

res = db.books.insert_many(books)
print("Inserted", len(res.inserted_ids))