require("dotenv").config();
const express = require("express");
const mongoose = require("mongoose");

//mongo setup
mongoose.connect(process.env.MONGO_URI, {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});
mongoose.set("useCreateIndex", true);

const noteSchema = mongoose.Schema({
  title: { type: String, required: true },
  body: { type: String, required: true },
});

const Note = mongoose.model("note", noteSchema);

//app setup
const app = express();
app.use(express.json({ type: "*/*" }));

//routes
app.get("/", (req, res) => {
  Note.find({}, (err, notes) => {
    if (!err) {
      res.status(200).send(notes);
      console.log("Found posts");
    }
  });
});

app.post("/", (req, res) => {
  const reqBody = req.body;
  console.log(reqBody, "got this");
  Note.create(reqBody, (err) => {
    if (!err) {
      res.status(201).send("Created");
      console.log("Created");
    } else {
      console.log(err);
      res.status(500).send(err);
    }
  });
});

app.listen(3000, () => {
  console.log("server listening on 3000");
});
