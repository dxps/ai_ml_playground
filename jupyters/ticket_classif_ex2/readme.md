##

### Prereqs

-   The standard packages: `pip install nltk spacy textblob wordcloud gensim`
-   The trained pipeline for English ([`en_core_web_sm`](https://spacy.io/models/en#en_core_web_sm) model: `spacy download en_core_web_sm`.
-   NLTK Data
    -   `sudo pip install nltk`
    -   `sudo python -m nltk.downloader -d /usr/share/nltk_data all` <br/>
        (as per [this](https://www.nltk.org/data.html), `/usr/share/nltk_data` is Unix (not macOS) specific location)
-   The input data: `cd ../data && tar zxf complaints-2021-05-14_08_16.json.tar.gz`
