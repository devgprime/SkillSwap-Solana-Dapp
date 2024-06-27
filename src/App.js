const express = require('express');
const app = express();
const config = require('config');
const mongoose = require('mongoose');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
const authRoutes = require('./routes/authRoutes');
const nftRoutes = require('./routes/nftRoutes');
const marketplaceRoutes = require('./routes/marketplaceRoutes');
const errorMiddleware = require('./middlewares/errorMiddleware');

// Middleware
app.use(express.json());
app.use(cors());
app.use(helmet());
app.use(morgan('dev'));

// Routes
app.use('/api/auth', authRoutes);
app.use('/api/nfts', nftRoutes);
app.use('/api/marketplace', marketplaceRoutes);

// Error handling middleware
app.use(errorMiddleware);

// Database connection
mongoose.connect(config.get('dbURI'), {
    useNewUrlParser: true,
    useUnifiedTopology: true,
    useCreateIndex: true,
    useFindAndModify: false
}).then(() => {
    console.log('Connected to MongoDB');
}).catch((err) => {
    console.error('Error connecting to MongoDB:', err.message);
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
