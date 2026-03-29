import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { useState } from 'react'
import { listPrincipals, listGovernanceArtifacts, reviewGovernanceVariant, promoteGovernanceVariant } from '../api'

export function Governance() {
  const queryClient = useQueryClient()
  const [selectedPrincipal, setSelectedPrincipal] = useState<string>('')

  // First fetch principals to select one
  const { data: principalsData } = useQuery({
    queryKey: ['principals'],
    queryFn: listPrincipals,
  })

  const { data, isLoading } = useQuery({
    queryKey: ['governance', selectedPrincipal],
    queryFn: () => listGovernanceArtifacts(selectedPrincipal),
    enabled: !!selectedPrincipal,
  })

  const reviewMutation = useMutation({
    mutationFn: ({ artifactId, status }: { artifactId: string; status: string }) =>
      reviewGovernanceVariant(selectedPrincipal, artifactId, status, 'Admin reviewed.'),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['governance', selectedPrincipal] })
    },
  })

  const promoteMutation = useMutation({
    mutationFn: (artifactId: string) =>
      promoteGovernanceVariant(selectedPrincipal, artifactId, 'Admin promoted to memory.'),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['governance', selectedPrincipal] })
    },
  })

  return (
    <div>
      <h1 className="page-title">Governance</h1>
      <p className="page-description">Review and promote synthesized artifacts.</p>

      <div style={{ marginTop: '24px', display: 'flex', gap: '16px', alignItems: 'center' }}>
        <select 
          className="input" 
          style={{ width: '300px' }} 
          value={selectedPrincipal} 
          onChange={(e) => setSelectedPrincipal(e.target.value)}
        >
          <option value="">Select Principal...</option>
          {principalsData?.principals?.map((p: any) => (
            <option key={p.principalId} value={p.principalId}>{p.principalId} ({p.userId})</option>
          ))}
        </select>
      </div>

      <div className="card" style={{ marginTop: '24px' }}>
        {isLoading && <p>Loading artifacts...</p>}
        {!selectedPrincipal && <p>Please select a principal to browse governance artifacts.</p>}
        
        {data?.items && (
          <div className="table-wrapper">
            <table>
              <thead>
                <tr>
                  <th>Artifact ID</th>
                  <th>Kind</th>
                  <th>Category</th>
                  <th>Content</th>
                  <th>Review Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {data.items.map((item: any) => (
                  <tr key={item.artifactId}>
                    <td title={item.artifactId}>{item.artifactId.slice(0, 8)}...</td>
                    <td>{item.artifactKind} - {item.artifactSubtype}</td>
                    <td>{item.category}</td>
                    <td>{item.contentPayload.content?.substring(0, 60)}...</td>
                    <td>
                      <span style={{ padding: '4px 8px', borderRadius: '4px', background: item.reviewStatus === 'approved' ? 'green' : item.reviewStatus === 'rejected' ? 'red' : 'rgba(255,255,255,0.1)' }}>
                        {item.reviewStatus}
                      </span>
                    </td>
                    <td style={{ display: 'flex', gap: '8px' }}>
                      <button 
                        className="btn btn-secondary"
                        onClick={() => reviewMutation.mutate({ artifactId: item.artifactId, status: 'approved' })}
                        disabled={item.reviewStatus === 'approved' || item.reviewStatus === 'promoted' || reviewMutation.isPending}
                      >
                        Approve
                      </button>
                      <button 
                        className="btn btn-secondary"
                        style={{ color: '#ff4d4f' }}
                        onClick={() => reviewMutation.mutate({ artifactId: item.artifactId, status: 'rejected' })}
                        disabled={item.reviewStatus === 'rejected' || item.reviewStatus === 'promoted' || reviewMutation.isPending}
                      >
                        Reject
                      </button>
                      <button 
                        className="btn btn-primary"
                        onClick={() => promoteMutation.mutate(item.artifactId)}
                        disabled={item.reviewStatus === 'promoted' || promoteMutation.isPending}
                      >
                        Promote
                      </button>
                    </td>
                  </tr>
                ))}
                {data.items.length === 0 && (
                  <tr>
                    <td colSpan={6} style={{ textAlign: 'center' }}>No candidates found for this principal.</td>
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
