use deepgram::transcription::prerecorded::response::Response;
use time::Duration;

pub trait CreateSubtitles {
    fn to_srt(&self, response: Response) -> Result<String, Box<dyn std::error::Error>>;
    fn seconds_to_timestamp(&self, time_string: i64) -> String;
}

pub struct Subtitles {}

impl CreateSubtitles for Subtitles {
    fn to_srt(&self, response: Response) -> Result<String, Box<dyn std::error::Error>> {
        let mut srt: String = "".to_owned();

        // TODO deal with unwrap
        // from javascript sdk
        // if (!this.results || !this.results.utterances) {
        //     throw new Error(
        //       "This function requires a transcript that was generated with the utterances feature."
        //     );
        //   }
        for (index, utterance) in response.results.utterances.unwrap().iter().enumerate() {
            // println!("{:?}", utterance);
            srt.push_str(
                format!(
                    "{}\n{} --> {}\n{}\n\n",
                    index + 1,
                    self.seconds_to_timestamp((utterance.start * 1000.) as i64),
                    self.seconds_to_timestamp((utterance.end * 1000.) as i64),
                    utterance.transcript
                )
                .as_str(),
            )
        }

        Ok(srt)
    }

    fn seconds_to_timestamp(&self, milliseconds: i64) -> String {
        // return new Date(seconds * 1000).toISOString().substr(11, 12);
        let d = Duration::milliseconds(milliseconds);
        format!(
            "{}:{}:{},{}",
            d.whole_hours(),
            d.whole_minutes() % 60,
            d.whole_seconds() % 60,
            d.whole_milliseconds() % 1000
        )
    }
}

#[test]
fn to_srt_test() {
    // note that this response has results.utterances[*].words as an empty array for brevity
    // a true response would also have that filled in
    let data = r#"
    {
        "metadata": {
          "transaction_key": "string",
          "request_id": "1e60a5d3-b237-4627-8334-7256e341ef67",
          "sha256": "string",
          "created": "string",
          "duration": 0,
          "channels": 0
        },
        "results": {
          "channels": [],
          "utterances": [
            {
              "start": 0.41915998,
              "end": 5.43012,
              "confidence": 0.88172823,
              "channel": 0,
              "transcript": "four score and seven years ago our fathers brought forth on this continent a new nation",
              "words": [],
              "id": "2d8211a4-3a5b-4053-8939-edf2b2b389fa"
            },
            {
              "start": 5.8882,
              "end": 9.880199,
              "confidence": 0.9834162,
              "channel": 0,
              "transcript": "conceived liberty and dedicated to the proposition that all men are created equal",
              "words": [],
              "id": "e88264de-a8cf-44e9-a7db-848ad5bab7a5"
            },
            {
              "start": 10.648263,
              "end": 333317.190998,
              "confidence": 0.9015952,
              "channel": 0,
              "transcript": "now we are engaged in a great civil war testing whether that nation or any nations open conceived and so dedicated can long endure",
              "words": [],
              "id": "1e60a5d3-b537-4627-8334-7256e341ef67"
            }
          ]
        }
      }
    "#;

    let resp: Response = serde_json::from_str(data).unwrap();

    let subtitles = Subtitles {};
    let srt = subtitles.to_srt(resp).expect("subtitle srt failed");
    assert_eq!(srt, "1\n0:0:0,419 --> 0:0:5,430\nfour score and seven years ago our fathers brought forth on this continent a new nation\n\n2\n0:0:5,888 --> 0:0:9,880\nconceived liberty and dedicated to the proposition that all men are created equal\n\n3\n0:0:10,648 --> 92:35:17,190\nnow we are engaged in a great civil war testing whether that nation or any nations open conceived and so dedicated can long endure\n\n");
}
