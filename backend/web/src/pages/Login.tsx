import { useState } from 'react'
import { useRouter } from '@tanstack/react-router'

export function Login() {
  const [token, setToken] = useState('')
  const router = useRouter()

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault()
    if (token.trim()) {
      sessionStorage.setItem('adminToken', token.trim())
      // navigate home
      router.navigate({ to: '/' })
    }
  }

  return (
    <div className="login-screen">
      <form className="login-card card" onSubmit={handleLogin}>
        <h2>Chronicle Admin</h2>
        <p className="page-description">Enter your admin token to continue.</p>
        <input
          type="password"
          className="input"
          placeholder="Admin Token"
          value={token}
          onChange={(e) => setToken(e.target.value)}
        />
        <button type="submit" className="btn btn-primary">
          Login
        </button>
      </form>
    </div>
  )
}
