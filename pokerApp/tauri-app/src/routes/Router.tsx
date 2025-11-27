import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import App from '@/App';
import LoginPages from '@/pages/auth/LoginPage';
import SignupPage from '@/pages/auth/SignupPage';
import MatchPage from '@/pages/game/MatchPage';
import HoldemPage from '@/pages/game/Holdem';
import GamePage from '@/pages/game/GamePage';
const AppRouter: React.FC = () => {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Navigate to="/login" replace />} />
        <Route path="/login" element={<LoginPages />} />
        <Route path="/signup" element={<SignupPage />} />
        <Route path="/app" element={<App />} />
        <Route path="/match" element={<MatchPage />} />
        <Route path="/holdem" element={<HoldemPage />} />
        <Route path="/game" element={<GamePage />} />
      </Routes>
    </Router>
  );
};

export default AppRouter;