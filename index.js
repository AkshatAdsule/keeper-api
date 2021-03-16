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
app.use(express.json({ type: "application/json" }));

// Rate Limiter
var RateLimit = require('express-rate-limit');
var limiter = new RateLimit({
  windowMs: 1*60*1000, // 1 minute
  max: 10
});

// apply rate limiter to all requests
app.use(limiter);

//routes
app.get("/", (_, res) => {
  Note.find({}, (error, notes) => {
    !error ? res.status(200).send(notes) : res.status(500).send(error);
  });
});

app.post("/", (req, res) => {
  let reqBody = req.body;
  Note.create(reqBody, (error) => {
    !error ? res.status(201).send("Created") : res.status(500).send(err);
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
