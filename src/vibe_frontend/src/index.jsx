import React from 'react';
import ReactDOM from 'react-dom/client';
import { HashRouter as Router, Routes, Route } from 'react-router-dom';

import App from "./components/App";
import '../assets/main.css';

import Landing from "./components/Landing";

import "@connect2ic/core/style.css"
import { createClient } from "@connect2ic/core"
import { Connect2ICProvider } from "@connect2ic/react"
import { defaultProviders } from "@connect2ic/core/providers"

const client = createClient({
  providers: defaultProviders,
  globalProviderConfig: {
    dev: false,
  },
});

const root = ReactDOM.createRoot(document.getElementById('root'));

root.render(
  <Connect2ICProvider client={client}>
    <Router>
      <Routes>
        <Route path="/" element={<App />}>
          <Route path="/" element={<Landing />} />
        </Route>
      </Routes>
    </Router>
  </Connect2ICProvider>
)
