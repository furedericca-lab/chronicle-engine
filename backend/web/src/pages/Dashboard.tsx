import { useQuery } from '@tanstack/react-query'
import { listPrincipals } from '../api'

export function Dashboard() {
  const { data, isLoading, error } = useQuery({
    queryKey: ['principals'],
    queryFn: listPrincipals,
  })

  return (
    <div>
      <h1 className="page-title">Dashboard</h1>
      <p className="page-description">Overview of Chronicle Engine.</p>
      
      <div className="card" style={{ marginTop: '24px' }}>
        <h3>Active Principals</h3>
        {isLoading && <p>Loading...</p>}
        {error && <p style={{ color: 'red' }}>Error: {error.message}</p>}
        
        {data?.principals && Array.isArray(data.principals) && (
          <div className="table-wrapper" style={{ marginTop: '16px' }}>
            <table>
              <thead>
                <tr>
                  <th>Principal ID</th>
                  <th>User ID</th>
                  <th>Agent ID</th>
                  <th>Memories</th>
                  <th>Transcripts</th>
                  <th>Jobs</th>
                </tr>
              </thead>
              <tbody>
                {data.principals.map((p: any) => (
                  <tr key={p.principalId} className="interactive">
                    <td>{p.principalId}</td>
                    <td>{p.userId}</td>
                    <td>{p.agentId}</td>
                    <td>{p.memoryCount}</td>
                    <td>{p.transcriptCount}</td>
                    <td>{p.distillJobCount}</td>
                  </tr>
                ))}
                {data.principals.length === 0 && (
                  <tr>
                    <td colSpan={6} style={{ textAlign: 'center' }}>No active principals.</td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  )
}
