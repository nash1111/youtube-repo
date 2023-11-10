import os
import requests
from dotenv import load_dotenv

# .envファイルから環境変数を読み込む
load_dotenv()

API_KEY = os.getenv("YOUTUBE_API_KEY")
PLAYLIST_ID = os.getenv("PLAYLIST_ID")
YOUTUBE_API_URL = "https://www.googleapis.com/youtube/v3/playlistItems"
YOUTUBE_ANALYTICS_API_URL = "https://youtubeanalytics.googleapis.com/v2/reports"


def get_youtube_videos(api_key, playlist_id):
    videos = []
    page_token = None

    while True:
        params = {
            "part": "snippet",
            "playlistId": playlist_id,
            "maxResults": 50,  # 1リクエストあたりの最大取得件数
            "pageToken": page_token,
            "key": api_key,
        }
        response = requests.get(YOUTUBE_API_URL, params=params)
        response.raise_for_status()
        data = response.json()

        videos.extend(data["items"])

        page_token = data.get("nextPageToken")
        if not page_token:
            break

    return videos


def get_video_analytics(api_key, video_id):
    params = {
        "ids": "channel==MINE",
        "startDate": "2021-01-01",
        "endDate": "2023-01-01",
        "metrics": "views",
        "dimensions": "video",
        "filters": f"video=={video_id}",
        "key": api_key,
    }
    response = requests.get(YOUTUBE_ANALYTICS_API_URL, params=params)
    return response.json()


def main():
    try:
        videos = get_youtube_videos(API_KEY, PLAYLIST_ID)
        video_ids = []
        for video in videos:
            title = video["snippet"]["title"]
            video_id = video["snippet"]["resourceId"]["videoId"]
            video_ids.append(video_id)
            print(f"title: {title}")
            print(f'video_id: {video["snippet"]["resourceId"]["videoId"]}')
        sample_analytics = get_video_analytics(API_KEY, video_ids[0])
        print(sample_analytics)

    except requests.RequestException as e:
        print(f"error: {e}")


if __name__ == "__main__":
    main()
