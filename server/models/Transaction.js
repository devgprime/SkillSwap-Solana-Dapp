const mongoose = require('mongoose');
const Schema = mongoose.Schema;

const transactionSchema = new Schema({
    buyer: { type: Schema.Types.ObjectId, ref: 'User', required: true },
    seller: { type: Schema.Types.ObjectId, ref: 'User', required: true },
    nft: { type: Schema.Types.ObjectId, ref: 'Nft', required: true },
    price: { type: Number, required: true },
    status: { type: String, enum: ['pending', 'completed', 'cancelled'], required: true }
}, { timestamps: true });

module.exports = mongoose.model('Transaction', transactionSchema);
