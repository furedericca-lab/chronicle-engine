import { useQuery } from '@tanstack/react-query'
import { useState } from 'react'
import { listPrincipals, listDistillJobs } from '../api'

export function DistillJobs() {
  const [selectedPrincipal, setSelectedPrincipal] = useState<string>('')
  
  const { data: principalsData } = useQuery({
    queryKey: ['principals'],
    queryFn: listPrincipals,
  })

  const { data: jobsData, isLoading, error } = useQuery({
    queryKey: ['distill_jobs', selectedPrincipal],
    queryFn: () => listDistillJobs(selectedPrincipal),
    enabled: !!selectedPrincipal,
  })

  const principals = principalsData?.principals || []

  return (
    <div>
      <h1 className="page-title">Distill Jobs</h1>
      <p className="page-description">Monitor background knowledge distillation jobs.</p>
      
      <div className="card" style={{ marginTop: '24px', marginBottom: '24px' }}>
        <label style={{ display: 'block', marginBottom: '8px' }}>Select Principal:</label>
        <select 
          value={selectedPrincipal} 
          onChange={(e) => setSelectedPrincipal(e.target.value)}
          style={{ padding: '8px', width: '100%', maxWidth: '400px' }}
        >
          <option value="">-- Select a Principal --</option>
          {principals.map((p: any) => (
            <option key={p.principalId} value={p.principalId}>
              {p.userId} / {p.agentId} ({p.principalId})
            </option>
          ))}
        </select>
      </div>

      {selectedPrincipal && (
        <div className="card">
          <h3>Jobs</h3>
          {isLoading && <p>Loading jobs...</p>}
          {error && <p style={{ color: 'red' }}>Error: {error.message}</p>}
          
          {jobsData?.items && (
            <div className="table-wrapper" style={{ marginTop: '16px' }}>
              <table>
                <thead>
                  <tr>
                    <th>Job ID</th>
                    <th>Status</th>
                    <th>Mode</th>
                    <th>Created At</th>
                    <th>Updated At</th>
                  </tr>
                </thead>
                <tbody>
                  {jobsData.items.map((j: any) => (
                    <tr key={j.jobId} className="interactive">
                      <td>{j.jobId}</td>
                      <td>{j.status}</td>
                      <td>{j.mode}</td>
                      <td>{new Date(j.createdAt).toLocaleString()}</td>
                      <td>{new Date(j.updatedAt).toLocaleString()}</td>
                    </tr>
                  ))}
                  {jobsData.items.length === 0 && (
                    <tr>
                      <td colSpan={5} style={{ textAlign: 'center' }}>No jobs found.</td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
