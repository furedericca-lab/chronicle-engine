import { Outlet, Link, useNavigate } from '@tanstack/react-router'
import { LayoutDashboard, Database, Activity, ScrollText, Scale, History, Settings as SettingsIcon, LogOut } from 'lucide-react'
import { useEffect, useState } from 'react'

export function RootLayout() {
  const [token, setToken] = useState<string | null>(null)
  const navigate = useNavigate()

  useEffect(() => {
    const t = sessionStorage.getItem('adminToken')
    if (!t) {
      navigate({ to: '/login' })
    } else {
      setToken(t)
    }
  }, [navigate])

  const logout = () => {
    sessionStorage.removeItem('adminToken')
    setToken(null)
    navigate({ to: '/login' })
  }

  if (!token) return null

  return (
    <div className="admin-shell">
      <aside className="sidebar">
        <div className="sidebar-header">
          Chronicle Engine Admin
        </div>
        <nav className="sidebar-nav">
          <Link to="/" className="nav-link" activeProps={{ className: 'active' }} activeOptions={{ exact: true }}>
            <LayoutDashboard size={18} /> Dashboard
          </Link>
          <Link to="/memories" className="nav-link" activeProps={{ className: 'active' }}>
            <Database size={18} /> Memories
          </Link>
          <Link to="/behavioral" className="nav-link" activeProps={{ className: 'active' }}>
            <Activity size={18} /> Behavioral
          </Link>
          <Link to="/recall" className="nav-link" activeProps={{ className: 'active' }}>
            <Database size={18} /> Recall Lab
          </Link>
          <Link to="/distill" className="nav-link" activeProps={{ className: 'active' }}>
            <Activity size={18} /> Distill Jobs
          </Link>
          <Link to="/transcripts" className="nav-link" activeProps={{ className: 'active' }}>
            <ScrollText size={18} /> Transcripts
          </Link>
          <Link to="/governance" className="nav-link" activeProps={{ className: 'active' }}>
            <Scale size={18} /> Governance
          </Link>
          <Link to="/audit" className="nav-link" activeProps={{ className: 'active' }}>
            <History size={18} /> Audit Log
          </Link>
          <Link to="/settings" className="nav-link" activeProps={{ className: 'active' }}>
            <SettingsIcon size={18} /> Settings
          </Link>
        </nav>
      </aside>
      
      <main className="main-content">
        <header className="topbar">
          <div>Admin Viewer</div>
          <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
            <button className="btn btn-secondary" onClick={logout} title="Logout">
              <LogOut size={16} />
            </button>
          </div>
        </header>
        <div className="page-wrapper">
          <Outlet />
        </div>
      </main>
    </div>
  )
}
