# this script will generate a CSV with ratings for albums
import random

albums = {
    "Meshuggah": ["Contradictions Collapse", "Destroy Erase Improve", "Chaosphere", "Nothing", "Catch Thirtythree", "ObZen", "Koloss", "The Violent Sleep of Reason", "Immutable"],
    "Vildhjarta": ["Omnilash", "Masstaden", "Thousands of Evils", "Masstaden Under Vaten", "Dar Skogen Sjunger"],
    "Humanity's Last Breath": ["Humanity's Last Breath", "Abyssal", "Vlade", "Ashen"]
}

means = {
    "Meshuggah": [1, 4, 2, 5, 5, 5, 3, 4, 4],
    "Vildhjarta": [1, 3, 4, 5, 4],
    "Humanity's Last Breath": [4, 2, 5, 4]
}

COUNT = 10000

if __name__ == "__main__":
    print("band,album,rating")
    artists = list(albums.keys())
    for i in range(COUNT):
        artist = random.choice(artists)
        artist_albums = albums[artist]
        album = random.choice(artist_albums)
        mean = means[artist][artist_albums.index(album)]
        rating = int(random.normalvariate(mu=mean, sigma=2))
        if rating < 0:
            rating = 0

        print(f"{artist},{album},{rating}")

