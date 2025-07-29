
# Bitcoin Price Visualization App

This is a full-stack application for visualizing Bitcoin prices in real time. It consists of a backend service, a frontend interface, and a PostgreSQL database. The prices are fetched periodically using the CoinGecko API, stored in a database, and served to a web-based frontend via WebSocket communication.

---

## Table of Contents

- [Project Overview](#project-overview)
- [Technologies Used](#technologies-used)
- [Architecture](#architecture)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Project](#running-the-project)
- [API Endpoints](#api-endpoints)
---

## Project Overview

This application consists of three major components:
1. **Frontend**: Displays a chart of Bitcoin prices using Chart.js.
2. **Backend**: Fetches Bitcoin prices from an external API, stores them in a PostgreSQL database, and broadcasts updates through WebSocket.
3. **Database**: PostgreSQL is used to store price data and timestamps.

---

## Technologies Used

- **Frontend**: HTML, JavaScript, Chart.js, Nginx (for serving static files)
- **Backend**: Rust, Axum (web framework), SQLx (database interaction), WebSocket
- **Database**: PostgreSQL
- **Containerization**: Docker, Docker Compose

---

## Architecture

The architecture of the application is as follows:

1. **Backend Service**:
   - Periodically polls the Bitcoin price using CoinGecko API.
   - Stores prices in the PostgreSQL database.
   - Sends real-time updates to clients over a WebSocket connection.
   - Serves as a middle layer between the frontend and the database.

2. **Frontend**:
   - Displays a dynamic chart of Bitcoin prices.
   - Communicates with the backend via WebSocket to receive updates.
   - Uses Chart.js for the interactive price chart.

3. **Database**:
   - Stores timestamped Bitcoin price data.

4. **Docker**:
   - All components are containerized for easier deployment.

---

## Features

- Real-time Bitcoin price updates using WebSocket.
- Historical data loading from the database into the price chart.
- Responsive and interactive chart powered by Chart.js.
- Modular and containerized setup for easy deployment.

---

## Prerequisites

- Docker
- Docker Compose

---

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/your-repo/bitcoin-price-visualization.git
   cd bitcoin-price-visualization
   ```

2. Ensure `Docker` and `Docker Compose` are installed on your system.

3. Build and run the application using Docker Compose:
   ```bash
   docker-compose up --build
   ```

---

## Running the Project

Once the setup is complete:

1. **Access Frontend**:
   - Visit [http://localhost:8080](http://localhost:8080) to view the Bitcoin price chart in your browser.

2. **Backend Service**:
   - The backend service runs on [http://localhost:3000](http://localhost:3000), serving the WebSocket endpoint at `/ws`.

3. **PostgreSQL Database**:
   - The database is accessible on `localhost:5432` (if you wish to inspect the data directly).

---

## API Endpoints

- **WebSocket**:
  - **Endpoint**: `/ws`
  - **Usage**: Subscribes to real-time Bitcoin price updates.
