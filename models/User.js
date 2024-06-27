const mongoose = require('mongoose');

const userSchema = new mongoose.Schema({
  username: { type: String, required: true },
  email: { type: String, required: true, unique: true },
  passwordHash: { type: String, required: true },
  bio: { type: String },
  skills: [{ type: String }],
  achievements: [{ type: String }],
  socialLinks: {
    twitter: { type: String },
    twitch: { type: String },
  },
  profilePicture: { type: String },
  createdAt: { type: Date, default: Date.now },
});

module.exports = mongoose.model('User', userSchema);

