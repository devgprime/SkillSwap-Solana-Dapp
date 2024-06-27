const express = require('express');
const nftController = require('../controllers/nftController');
const authMiddleware = require('../middlewares/authMiddleware');

const router = express.Router();

router.post('/create', authMiddleware, nftController.createNft);
router.get('/', nftController.listNfts);
router.get('/:id', nftController.getNftById);

module.exports = router;
