const API_BASE = '/admin/api'

function idempotencyKey() {
  return typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
    ? crypto.randomUUID()
    : `admin-${Date.now()}-${Math.random().toString(16).slice(2)}`
}

export async function fetchWithAuth(url: string, options: RequestInit = {}) {
  const token = sessionStorage.getItem('adminToken')
  const headers = new Headers(options.headers)
  
  if (token) {
    headers.set('Authorization', `Bearer ${token}`)
  }
  if (!headers.has('Content-Type') && !(options.body instanceof FormData)) {
    headers.set('Content-Type', 'application/json')
  }

  const res = await fetch(`${API_BASE}${url}`, { ...options, headers })
  if (!res.ok) {
    const errorText = await res.text()
    if (res.status === 401 || res.status === 403) {
      sessionStorage.removeItem('adminToken')
      window.location.href = '/admin/login' // fallback for deep unauthorized responses
    }
    throw new Error(errorText || res.statusText)
  }
  
  return res.json()
}

// -- Principals
export const listPrincipals = () => fetchWithAuth('/principals')

// -- Memories
export const listMemories = (principalId: string) => 
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/memories`)

export const getMemory = (principalId: string, memoryId: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/memories/${encodeURIComponent(memoryId)}`)

export const deleteMemory = (principalId: string, memoryId: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/memories/${encodeURIComponent(memoryId)}`, {
    method: 'DELETE',
  })

// -- Distill Jobs
export const listDistillJobs = (principalId: string) => 
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/distill/jobs`)

export const getDistillJob = (principalId: string, jobId: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/distill/jobs/${encodeURIComponent(jobId)}`)

// -- Transcripts
export const listTranscripts = (principalId: string) => 
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/session-transcripts`)

export const getTranscript = (principalId: string, transcriptId: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/session-transcripts/${encodeURIComponent(transcriptId)}`)

// -- Governance
export const listGovernanceArtifacts = (principalId: string) => 
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/governance/artifacts`)

export const reviewGovernanceVariant = (principalId: string, artifactId: string, status: string, note?: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/governance/artifacts/${encodeURIComponent(artifactId)}/review`, {
    method: 'POST',
    headers: { 'Idempotency-Key': idempotencyKey() },
    body: JSON.stringify({ reviewStatus: status, reviewerNote: note }),
  })

export const promoteGovernanceVariant = (principalId: string, artifactId: string, note?: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/governance/artifacts/${encodeURIComponent(artifactId)}/promote`, {
    method: 'POST',
    headers: { 'Idempotency-Key': idempotencyKey() },
    body: JSON.stringify({ reviewerNote: note }),
  })

// -- Recall Simulate
export const recallSimulate = (principalId: string, query: string, mode: string) =>
  fetchWithAuth(`/principals/${encodeURIComponent(principalId)}/recall/simulate`, {
    method: 'POST',
    body: JSON.stringify({ query, mode }),
  })

// -- Audit & Settings
export const getAuditLog = () => fetchWithAuth('/audit-log?limit=200')
export const getSettings = () => fetchWithAuth('/settings/runtime-config')
export const updateSettings = (configToml: string) => 
  fetchWithAuth('/settings/runtime-config', { method: 'PUT', body: JSON.stringify({ configToml }) })
