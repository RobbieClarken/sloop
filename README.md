# sloop

Command-line tool to simplify importing audiobooks into a podcast app. Given a list of audio
files, sloop will generate an RSS feed and upload this feed and the audio files to S3.

## Usage

1. Download and install the [awscli](https://aws.amazon.com/cli/) and run `aws configure` to
   setup your environment.
2. Install sloop:
   
    ```
    $ cargo install sloop
    ```
    
3. Run `sloop feed` to generate your feed. Include the `--upload` option to also upload
   to S3:
   
   ```
   $ sloop feed --title Candide --out feed.xml --upload --bucket candide-a5e21f --region ap-southeast-2 Chapter_*.mp3    
   Uploading feed.xml
   Uploading Chapter_01.mp3
   Uploading Chapter_02.mp3
   ...
   Uploading Chapter_29.mp3
   Uploading Chapter_30.mp3
   Upload complete
   Podcast available at https://candide-a5e21f.s3-ap-southeast-2.amazonaws.com/feed.xml
   ```
