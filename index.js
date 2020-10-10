require("dotenv").config();
const express = require("express");
const mongoose = require("mongoose");
const bodyparser = require("body-parser");

mongoose.connect(process.env.MONGO_URI, {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});
mongoose.set("useCreateIndex", true);

const noteSchema = mongoose.Schema({
  id: String,
  title: String,
  body: String,
});

const Note = mongoose.model("note", noteSchema);

const app = express();
app.use(bodyparser.urlencoded({ extended: true }));

app.get("/", (req, res) => {
  Note.find({}, (err, notes) => {
    if (!err) {
      res.status(200).send(notes);
      console.log("Found posts");
    }
  });
});

app.post("/", (req, res) => {
  Note.create(
    {
      id: "1",
      title: "Hello",
      body: "World",
    },
    (err) => {
      if (!err) {
        res.status(201).send("Created");
        console.log("Created");
      }
    }
  );
});

app.listen(3000, () => {
  console.log("server listening on 3000");
});
