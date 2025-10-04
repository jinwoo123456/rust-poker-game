import React, { useState } from 'react';
import '@/styles/pages/auth/auth-pages.css';
import axios from 'axios';
import { API_URL } from '@/utils/path';
import chipsLogo from '@/assets/images/poker-chips.png';
import Cookie from 'js-cookie';
import { Link, useNavigate } from 'react-router-dom';
export default function LoginPages() {
  const [userid, setUserId] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();
  const [cookie, setCookie] = useState(Cookie.get('userid') || '');
  // async function getUsers() {
  //   try {
  //     const res = await axios.get("https://jsonplaceholder.typicode.com/users");
  //     console.log(res.data); // 응답 데이터
  //   } catch (err) {
  //     console.error(err); // 에러 처리
  //   }
  // }
  async function handleLogin(event: React.FormEvent) {
    event.preventDefault();
    try {
      const response = await axios.post(`${API_URL}/login`, {
        userid,
        password,
      });
      console.log(response.data);
      Cookie.set('userid', response.data.userid, { expires: 1 });
      navigate('/match');
    } catch (error) {
      alert('로그인에 실패했습니다. 아이디와 비밀번호를 확인하세요.');
      console.error(error);
    }
  }

  return (
    <div className="login-container">
      
      <h1>Play For Holdem</h1>
      
      <form onSubmit={handleLogin} className="login-form">
        <div className="form-group">
          <label htmlFor="userid">사용자명:</label>
          <input
            type="text"
            id="userid"
            value={userid}
            onChange={(e) => setUserId(e.target.value)}
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
        <button className='login-button' type="submit">로그인</button>
      </form>
      <span style={{ fontSize: '12px', color: '#888' }}>계정이 없으신가요?</span>
      <Link to="/signup" style={{  color: '#777' }}>회원가입</Link>
      </div>
    
  );
};

