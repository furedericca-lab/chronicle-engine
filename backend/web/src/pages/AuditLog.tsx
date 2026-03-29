import { useQuery } from '@tanstack/react-query'
import { getAuditLog } from '../api'

export function AuditLog() {
  const { data, isLoading } = useQuery({
    queryKey: ['audit'],
    queryFn: getAuditLog,
  })

  return (
    <div>
      <h1 className="page-title">Audit Log</h1>
      <p className="page-description">History of sensitive administrative actions.</p>
      
      <div className="card" style={{ marginTop: '24px' }}>
        {isLoading && <p>Loading audit logs...</p>}
        {data?.items && (
          <div className="table-wrapper">
            <table>
              <thead>
                <tr>
                  <th>Event ID</th>
                  <th>Timestamp</th>
                  <th>Action</th>
                  <th>Subject</th>
                  <th>Target Type</th>
                  <th>Target ID</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {data.items.map((log: any) => (
                  <tr key={log.id} className="interactive">
                    <td title={log.id}>{log.id.slice(0,8)}...</td>
                    <td>{new Date(log.timestamp).toLocaleString()}</td>
                    <td>{log.action}</td>
                    <td>{log.adminSubject}</td>
                    <td>{log.targetResourceKind || '-'}</td>
                    <td>{log.targetResourceId ? log.targetResourceId.slice(0,8) + '...' : '-'}</td>
                    <td>{log.outcome}</td>
                  </tr>
                ))}
                {data.items.length === 0 && (
                  <tr>
                    <td colSpan={7} style={{ textAlign: 'center' }}>No audit trails exist yet.</td>
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
