const mongoose = require('mongoose');
const Schema = mongoose.Schema;

const nftSchema = new Schema({
    owner: { type: Schema.Types.ObjectId, ref: 'User', required: true },
    skillData: {
        player_id: { type: String, required: true },
        game_name: { type: String, required: true },
        rank: { type: Number, required: true },
        playstyle: { type: String, required: true },
        achievements: [{ type: String, required: true }],
        social_links: {
            twitter: { type: String, required: false },
            twitch: { type: String, required: false }
        }
    },
    price: { type: Number, required: true },
    is_listed: { type: Boolean, default: false },
}, { timestamps: true });

module.exports = mongoose.model('Nft', nftSchema);
