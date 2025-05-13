# AI Text Paraphraser

## Links:
### Frontend: (https://visionary-heliotrope-955ec6.netlify.app/)
### Backend: (https://text-paraphraser-hb2k.onrender.com/paraphrase)

This is a web application that allows users to paraphrase text using AI. Built with React/TypeScript frontend and Rust/Actix-web backend, leveraging the OpenRouter API.



## Development Environment Setup

This guide will help you set up the complete development environment for both frontend and backend components.

### Prerequisites

Before starting, make sure you have the following installed:

- **Node.js** (v20.16.0) and npm (v10.8.1)
- **Rust** (1.28.2) via [rustup](https://rustup.rs/)
- **Git** for version control
- An **OpenRouter API key** ([Get one here](https://openrouter.ai/))

### Backend Setup

1. Clone the backend repository:
```bash
git clone https://github.com/joshualine/ai-paraphraser-api.git
```

2. Navigate to the backend directory:
   ```bash
   cd ai-paraphraser-api
   ```

3. Create a `.env` file:
   ```bash
   echo "OPENROUTER_API_KEY=your_openrouter_api_key_here" > .env
   ```

4. Run the development environment:

```bash
# Build and run the project in development mode
cargo run
```

The server will start at http://localhost:8080

### Frontend Setup (React/TypeScript)

1. Clone the frontend repository:
```bash
git clone https://github.com/joshualine/ai-paraphraser-frontend.git
```

2. Navigate to the frontend directory:
   ```bash
   # From the project root
   cd ai-paraphraser-frontend
   ```

3. Install npm packages:
   ```bash
   npm install
   ```

4. Run the development environment:

```bash
npm run dev
```

The Vite dev server will start at http://localhost:5173

### 4. Working with the Full Stack

With both servers running (backend on port 8080 and frontend on port 5173):

1. The frontend will proxy API requests to the backend through Vite's proxy configuration
2. Any changes to the frontend code will trigger hot-reloading
3. Changes to the Rust backend will require restarting the server with `cargo run`

### 5. Development Workflow Tips

#### Frontend Development

- All React components are in `frontend/src/components/`
- Main state is managed in `App.tsx`
- CSS styling is inline

To add a new component:
1. Create a new file in the components directory
2. Import and use it in App.tsx or another component
3. Add any necessary state or props

#### Backend Development

- Main API logic is in `backend/src/main.rs`
- The paraphrase endpoint handles the core functionality

To modify the AI prompt or behavior:
1. Update the `OpenRouterRequest` structure in the `paraphrase` function
2. Adjust the system and user messages to change AI instructions


### 6. Running Tests

#### Frontend Tests

```bash
cd frontend
npm test
```

#### Backend Tests

```bash
cd backend
cargo test
```



## Common Development Issues

### Backend Issues

1. **"OPENROUTER_API_KEY not set in environment"**
   - Solution: Make sure your `.env` file exists in the backend directory with the correct API key

2. **Port already in use**
   - Solution: Change the port in `main.rs` or kill the process using the port

### Frontend Issues

1. **Proxy not working**
   - Solution: Ensure the backend is running and the proxy settings in `vite.config.ts` match your backend URL

2. **TypeScript errors**
   - Solution: Run `npm run lint` to identify and fix issues

## Architecture Overview

The application uses a client-server architecture with:

- **React Frontend**: Handles user interaction and display
- **Rust Backend**: Manages API requests and AI service integration
- **OpenRouter API**: Provides AI text processing capabilities

Requests flow from the frontend through our backend to the OpenRouter API, with responses following the reverse path.


# My Architecture Choices

## High-Level Architecture

This project employs a clean client-server architecture with a clear separation of concerns:

1. **Frontend**: A React/TypeScript single-page application
2. **Backend**: A Rust-based RESTful API service
3. **External Service**: OpenRouter AI API for text processing


The application follows a typical web application request flow:
- User inputs text on the frontend
- Frontend sends a request to our backend
- Backend processes and forwards the request to OpenRouter
- Results flow back through the same path to the user interface

## Frontend Architecture Choices

### React with TypeScript

- **Component-Based Structure**: Breaking the UI into modular, reusable components (Header, TextArea, ParaphraseButton, ResultsArea)
- **TypeScript Integration**: Strict typing for better developer experience and fewer runtime errors
- **State Management**: Using React's useState hooks for simple state management without external libraries
- **Vite Build System**: Chosen for its speed and modern development experience

### Frontend Design Patterns

- **Container/Presentational Pattern**: App.tsx serves as a container component while other components are primarily presentational
- **Prop Drilling**: Used for this relatively small application rather than introducing context or state management libraries
- **Error Handling**: Centralized in the App component with appropriate UI feedback

## Backend Architecture Choices

### Rust with Actix-web

- **Actix-web Framework**: High-performance, type-safe web framework
- **Asynchronous Processing**: Using Tokio runtime for efficient handling of concurrent requests
- **Strong Type Safety**: Leveraging Rust's type system for robust request/response handling
- **Middleware Approach**: Using CORS middleware for proper cross-origin handling

### API Design

- **RESTful Principles**: Clear endpoint design with appropriate HTTP methods
- **JSON Serialization/Deserialization**: Using serde for type-safe data conversion
- **Error Standardization**: Consistent error response format
- **API Key Security**: Proper handling of API credentials using environment variables and secrets

## Integration with OpenRouter AI

- **HTTP Client Abstraction**: Using reqwest for clean API interactions
- **Prompt Engineering**: Carefully crafted system and user prompts for optimal paraphrasing results
- **Error Handling**: Proper cascading of errors from the external API to the frontend

## Deployment Strategy

- **Independent Deployments**: Frontend and backend were deployed separately: Backend on Render and Frontend on Netlify
- **Environment-Specific Configuration**: Using .env and secret management

These architecture choices provide a balance of performance, maintainability, and developer experience while keeping the system simple enough for a focused application.


# ⚠️ Challenges Encountered while building this

During the development of the AI Paraphrasing Web Application, I was faced with the following challanges:

---

### 🔐 1. API Integration

- **Issue:** The OpenRouter API required specific request formatting, including role-based messages and proper JSON structure.
- **Solution:** Used structured request models in Rust with `serde` to ensure the payload matched OpenAI’s expected format.

---

### 🌐 2. Cross-Origin Resource Sharing (CORS)

- **Issue:** During local development, the frontend (`localhost:5173`) could not make API requests to the backend (`localhost:8080`) due to CORS restrictions.
- **Solution:** CORS middleware configuration was considered for the Actix-web server. It’s recommended to add `actix-cors` in production or when running frontend/backend separately.

---

### 🔄 3. Asynchronous Handling in Rust

- **Issue:** Making asynchronous HTTP requests (to OpenAI) within Actix-web required a proper understanding of async/await patterns and the use of the `tokio` runtime.
- **Solution:** Leveraged `reqwest` with `tokio` and ensured all async operations were properly awaited and handled with error management.

---

### 🧪 4. Inconsistent API Responses

- **Issue:** OpenAI's responses vary in structure and content. This caused errors when trying to parse or display paraphrased text reliably.
- **Solution:** Added safe JSON parsing with fallback defaults to handle unexpected or malformed responses.

---

### 📦 5. Environment Variable Management

- **Issue:** Backend required an API key stored securely. Forgetting to set `OPENAI_API_KEY` resulted in runtime panics.
- **Solution:** Used the `dotenv` crate in Rust and included `.env` in `.gitignore`. Added clear setup instructions in the README.

---


These challenges provided valuable learning opportunities, especially in areas like Rust’s asynchronous ecosystem, full-stack integration, and safe API consumption.
