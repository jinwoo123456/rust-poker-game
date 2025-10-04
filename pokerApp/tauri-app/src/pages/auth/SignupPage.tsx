import React, { useState } from 'react';
import '@/styles/pages/auth/auth-pages.css';
import axios from 'axios';
import { API_URL } from '@/utils/path';
import chipsLogo from '@/assets/images/poker-chips.png';
import { Link, useNavigate } from 'react-router-dom';
export default function SignupPage() {
  const [userid, setUserid] = useState('');
  const [password, setPassword] = useState('');
  const [rePassword, setRePassword] = useState('');
  
  const navigate = useNavigate();
  // async function getUsers() {
  //   try {
  //     const res = await axios.get("https://jsonplaceholder.typicode.com/users");
  //     console.log(res.data); // 응답 데이터
  //   } catch (err) {
  //     console.error(err); // 에러 처리
  //   }
  // }
  async function handleSignup(event: React.FormEvent) {
    event.preventDefault();
    if (userid.trim() === '' || password.trim() === '') {
        alert('사용자명과 비밀번호를 모두 입력하세요.');
        return;
    }
    if (password !== rePassword) {
        alert('비밀번호가 일치하지 않습니다.');
        return;
    }
    try {
      const response = await axios.post(`${API_URL}/signup`, {
        userid,
        password,
        
      });
      alert('회원가입이 완료되었습니다. 로그인 페이지로 이동합니다.');
      navigate('/login');
      console.log(response.data);
    } catch (error) {
      alert('이미 가입된 아이디가 있습니다.');
      console.error(error);
    }
  }

  return (
    <div className="login-container">
      
      <h1>Play For Holdem</h1>
      
      <form onSubmit={handleSignup} className="login-form">
        <div className="form-group">
          <label htmlFor="userid">아이디:</label>
          <input
            type="text"
            id="userid"
            value={userid}
            onChange={(e) => setUserid(e.target.value)}
            placeholder="사용자명을 입력하세요"
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="password">비밀번호:</label>
          <input
            type="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="비밀번호를 입력하세요"
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="password">비밀번호 재입력:</label>
          <input
            type="password"
            id="re-password"
            value={rePassword}
            onChange={(e) => setRePassword(e.target.value)}
            placeholder="비밀번호를 입력하세요"
            required
          />
        </div>
        <button className='login-button' type="submit">로그인</button>
      </form>
      <span style={{ fontSize: '12px', color: '#888' }}>아이디가 있으신가요?</span>
      <Link to="/login" style={{  color: '#777' }}>로그인</Link>
      </div>
    
  );
};

