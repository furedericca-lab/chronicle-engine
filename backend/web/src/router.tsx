import { createRootRoute, createRoute, createRouter, Outlet } from '@tanstack/react-router'
import { RootLayout } from './components/RootLayout'
import { Dashboard } from './pages/Dashboard'
import { Memories } from './pages/Memories'
import { Behavioral } from './pages/Behavioral'
import { RecallLab } from './pages/RecallLab'
import { DistillJobs } from './pages/DistillJobs'
import { Transcripts } from './pages/Transcripts'
import { Governance } from './pages/Governance'
import { AuditLog } from './pages/AuditLog'
import { Settings } from './pages/Settings'
import { Login } from './pages/Login'

export const rootRoute = createRootRoute({
  component: () => <Outlet />,
})

export const authRoute = createRoute({
  getParentRoute: () => rootRoute,
  id: 'auth',
  component: RootLayout,
})

const loginRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/login',
  component: Login,
})

const indexRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/',
  component: Dashboard,
})

const memoriesRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/memories',
  component: Memories,
})

const behavioralRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/behavioral',
  component: Behavioral,
})

const recallRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/recall',
  component: RecallLab,
})

const distillRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/distill',
  component: DistillJobs,
})

const transcriptsRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/transcripts',
  component: Transcripts,
})

const governanceRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/governance',
  component: Governance,
})

const auditRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/audit',
  component: AuditLog,
})

const settingsRoute = createRoute({
  getParentRoute: () => authRoute,
  path: '/settings',
  component: Settings,
})

const routeTree = rootRoute.addChildren([
  loginRoute,
  authRoute.addChildren([
    indexRoute,
    memoriesRoute,
    behavioralRoute,
    recallRoute,
    distillRoute,
    transcriptsRoute,
    governanceRoute,
    auditRoute,
    settingsRoute,
  ]),
])

export const router = createRouter({ routeTree })
