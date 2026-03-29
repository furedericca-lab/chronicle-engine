import { useQuery } from '@tanstack/react-query'
import { useState } from 'react'
import { listPrincipals, listMemories } from '../api'

export function Memories() {
  const [selectedPrincipal, setSelectedPrincipal] = useState<string>('')
  
  const { data: principalsData } = useQuery({
    queryKey: ['principals'],
    queryFn: listPrincipals,
  })

  const { data: memoriesData, isLoading, error } = useQuery({
    queryKey: ['memories', selectedPrincipal],
    queryFn: () => listMemories(selectedPrincipal),
    enabled: !!selectedPrincipal,
  })

  const principals = principalsData?.principals || []

  return (
    <div>
      <h1 className="page-title">Memories</h1>
      <p className="page-description">Browse memories for a selected principal.</p>
      
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
          <h3>Memories</h3>
          {isLoading && <p>Loading memories...</p>}
          {error && <p style={{ color: 'red' }}>Error: {error.message}</p>}
          
          {memoriesData?.items && (
            <div className="table-wrapper" style={{ marginTop: '16px' }}>
              <table>
                <thead>
                  <tr>
                    <th>ID</th>
                    <th>Category</th>
                    <th>Preview</th>
                    <th>Source</th>
                    <th>Created At</th>
                  </tr>
                </thead>
                <tbody>
                  {memoriesData.items.map((m: any) => (
                    <tr key={m.id} className="interactive">
                      <td>{m.id.substring(0, 8)}...</td>
                      <td>{m.category}</td>
                      <td>{m.textPreview}</td>
                      <td>{m.source}</td>
                      <td>{new Date(m.createdAt).toLocaleString()}</td>
                    </tr>
                  ))}
                  {memoriesData.items.length === 0 && (
                    <tr>
                      <td colSpan={5} style={{ textAlign: 'center' }}>No memories found.</td>
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
