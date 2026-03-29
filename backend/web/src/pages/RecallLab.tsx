import { useQuery } from '@tanstack/react-query'
import { useState } from 'react'
import { listPrincipals, recallSimulate } from '../api'

export function RecallLab() {
  const [selectedPrincipal, setSelectedPrincipal] = useState<string>('')
  const [query, setQuery] = useState<string>('')
  const [mode, setMode] = useState<string>('generic')
  const [results, setResults] = useState<any>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  
  const { data: principalsData } = useQuery({
    queryKey: ['principals'],
    queryFn: listPrincipals,
  })

  const principals = principalsData?.principals || []

  const handleSimulate = async () => {
    if (!selectedPrincipal || !query) return
    setLoading(true)
    setError(null)
    try {
      const data = await recallSimulate(selectedPrincipal, query, mode)
      setResults(data)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div>
      <h1 className="page-title">Recall Lab</h1>
      <p className="page-description">Test and trace retrieval logic.</p>
      
      <div className="card" style={{ marginTop: '24px', marginBottom: '24px' }}>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', maxWidth: '600px' }}>
          <div>
            <label style={{ display: 'block', marginBottom: '8px' }}>Select Principal:</label>
            <select 
              value={selectedPrincipal} 
              onChange={(e) => setSelectedPrincipal(e.target.value)}
              style={{ padding: '8px', width: '100%' }}
            >
              <option value="">-- Select a Principal --</option>
              {principals.map((p: any) => (
                <option key={p.principalId} value={p.principalId}>
                  {p.userId} / {p.agentId} ({p.principalId})
                </option>
              ))}
            </select>
          </div>
          <div>
            <label style={{ display: 'block', marginBottom: '8px' }}>Mode:</label>
            <select 
              value={mode} 
              onChange={(e) => setMode(e.target.value)}
              style={{ padding: '8px', width: '100%' }}
            >
              <option value="generic">Generic</option>
              <option value="behavioral">Behavioral</option>
            </select>
          </div>
          <div>
            <label style={{ display: 'block', marginBottom: '8px' }}>Query:</label>
            <textarea 
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              style={{ padding: '8px', width: '100%', minHeight: '80px', fontFamily: 'monospace' }}
              placeholder="Enter search text..."
            />
          </div>
          <button 
            onClick={handleSimulate} 
            disabled={loading || !selectedPrincipal || !query}
            style={{ padding: '8px 16px', background: '#3b82f6', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
          >
            {loading ? 'Simulating...' : 'Simulate Recall'}
          </button>
        </div>
      </div>

      {error && <p style={{ color: 'red' }}>Error: {error}</p>}

      {results && (
        <div className="card">
          <h3>Results</h3>
          <p>Mode: {results.appliedFilters?.mode}</p>
          <div className="table-wrapper" style={{ marginTop: '16px' }}>
            <table>
              <thead>
                <tr>
                  <th>Score</th>
                  <th>ID</th>
                  <th>Text</th>
                </tr>
              </thead>
              <tbody>
                {results.results?.map((r: any, i: number) => (
                  <tr key={i}>
                    <td>{r.score?.toFixed(3) || '-'}</td>
                    <td>{r.id?.substring(0, 8)}</td>
                    <td>{r.text}</td>
                  </tr>
                ))}
                {(!results.results || results.results.length === 0) && (
                  <tr><td colSpan={3} style={{ textAlign: 'center' }}>No results</td></tr>
                )}
              </tbody>
            </table>
          </div>
          <h4 style={{ marginTop: '24px' }}>Trace</h4>
          <pre style={{ background: '#1e293b', padding: '16px', overflowX: 'auto', fontSize: '12px' }}>
            {JSON.stringify(results.trace, null, 2)}
          </pre>
        </div>
      )}
    </div>
  )
}
