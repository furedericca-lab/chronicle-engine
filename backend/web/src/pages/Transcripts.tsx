import { useQuery } from '@tanstack/react-query'
import { useState } from 'react'
import { listPrincipals, listTranscripts } from '../api'

export function Transcripts() {
  const [selectedPrincipal, setSelectedPrincipal] = useState<string>('')
  
  const { data: principalsData } = useQuery({
    queryKey: ['principals'],
    queryFn: listPrincipals,
  })

  const { data: transcriptsData, isLoading, error } = useQuery({
    queryKey: ['transcripts', selectedPrincipal],
    queryFn: () => listTranscripts(selectedPrincipal),
    enabled: !!selectedPrincipal,
  })

  const principals = principalsData?.principals || []

  return (
    <div>
      <h1 className="page-title">Transcripts</h1>
      <p className="page-description">View session transcripts.</p>
      
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
          <h3>Session Transcripts</h3>
          {isLoading && <p>Loading transcripts...</p>}
          {error && <p style={{ color: 'red' }}>Error: {error.message}</p>}
          
          {transcriptsData?.items && (
            <div className="table-wrapper" style={{ marginTop: '16px' }}>
              <table>
                <thead>
                  <tr>
                    <th>Transcript ID</th>
                    <th>Session Key</th>
                    <th>Session ID</th>
                    <th>Messages</th>
                    <th>First Timestamp</th>
                    <th>Last Timestamp</th>
                  </tr>
                </thead>
                <tbody>
                  {transcriptsData.items.map((t: any) => (
                    <tr key={t.transcriptId} className="interactive">
                      <td>{t.transcriptId.substring(0, 12)}...</td>
                      <td>{t.sessionKey}</td>
                      <td>{t.sessionId}</td>
                      <td>{t.messageCount}</td>
                      <td>{new Date(t.firstTimestamp).toLocaleString()}</td>
                      <td>{new Date(t.lastTimestamp).toLocaleString()}</td>
                    </tr>
                  ))}
                  {transcriptsData.items.length === 0 && (
                    <tr>
                      <td colSpan={6} style={{ textAlign: 'center' }}>No transcripts found.</td>
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
