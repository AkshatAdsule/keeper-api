require("dotenv").config();
const express = require("express");
const mongoose = require("mongoose");
const cors = require("cors");

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
app.use(express.json({ type: "application/json" }));
app.use(cors());

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
  let reqBody = req.body;
  Note.create(reqBody, (err) => {
    !err ? res.status(201).send("Created") : res.status(500).send(err);
  });
});

app.delete("/:id", (req, res) => {
  console.log(req.params.id);
  Note.deleteOne({ _id: req.params.id }, (error) => {
    !error ? res.status(202).send("deleted") : res.status(500).send(error);
  });
});

app.listen(process.env.PORT || 3001, () => {
  console.log("server listening on 3000");
});
