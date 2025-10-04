import { API_URL } from "@/utils/path";
import { LoadingOverlay } from "@/utils/loading";
import axios from "axios";
import { useEffect, useState } from "react";
import Cookie from 'js-cookie';
import { Navigate, useNavigate } from "react-router-dom";
type Player = {
    userid: string;
    status: number;
    match_at: string;
};

export default function MatchPage() {

    const [matchList, setMatchList] = useState<Player[]>([]);
    const [matchStatus] = useState(1);
    const [isLoading, setIsLoading] = useState(false);
    const navigate = useNavigate(); 
    const getKstIsoString = () => {
        const now = new Date();
        const kst = new Date(now.getTime() + 9 * 60 * 60 * 1000);
        return kst.toISOString().replace('Z', '+09:00');
    };


    async function handleStartGame(playerId: string) {

        try{
            setIsLoading(true);
            const resp = await axios.post(`${API_URL}/holdem/start`, {
                my_player_id: Cookie.get('userid'),
                select_player_id : playerId,
            });
            console.log('게임 시작 결과:', resp.data);
            navigate('/holdem');
        } catch (error) {
            console.error('게임 시작 중 오류 발생:', error);
        } finally {
            setIsLoading(false);
        }
    }


    async function handleMatch() {
        try {
            setIsLoading(true);
            const resp = await axios.post(`${API_URL}/match/create`, {
                userid: Cookie.get('userid'),
                status: matchStatus,
                match_at: getKstIsoString(),
            });
            console.log('매칭 생성 결과:', resp.data);
            await getMatchList();

        } catch (error) {
            console.error('매칭 생성 중 오류 발생:', error);
        } finally {
            setIsLoading(false);
        }
    }
    async function getMatchList() {
        try {
            const resp = await axios.get(`${API_URL}/match/show`);
            console.log('매칭된 플레이어 목록:', resp.data);
            if (Array.isArray(resp.data.players)) {
                setMatchList(resp.data.players);
            } else {
                console.warn('플레이어 목록이 올바르지 않습니다:', resp.data);
                setMatchList([]);
            }
        } catch (error) {
            console.error('매칭된 플레이어 목록을 가져오는 중 오류 발생:', error);
        }
    };
 
    useEffect(() => {
        getMatchList();
        console.log("매칭 페이지가 로드되었습니다.");
    }, []);

    return (
        <>
            <LoadingOverlay isLoading={isLoading} />
            <div className="container py-4">
                <div className="d-flex justify-content-between align-items-center mb-3">
                    <h1 className="h3 mb-0">매칭 페이지</h1>
                    <button className="btn btn-primary" onClick={handleMatch}>
                        매칭 잡기
                    </button>
                </div>
                <div className="card shadow-sm">
                    <div className="card-body p-0">
                        <table className="table table-striped table-hover mb-0 text-center align-middle">
                            <thead className="table-dark">
                                <tr>
                                    <th scope="col">사용자명</th>
                                    <th scope="col">매치 시간</th>
                                    <th scope="col">게임하기</th>
                                </tr>
                            </thead>
                            <tbody>
                                {matchList.length > 0 ? (
                                    matchList.map((player, index) => (
                                        <tr key={index}>
                                            <td className="fw-semibold">{player.userid}</td>
                                            <td>{new Date(player.match_at).toLocaleString('ko-KR', { timeZone: 'Asia/Seoul' })}</td>
                                            <td>
                                                <input type="hidden" value={player.userid} />
                                                <button onClick={()=> handleStartGame(player.userid)} className="btn btn-sm btn-primary">게임하기</button>
                                            </td>
                                        </tr>
                                    ))
                                ) : (
                                    <tr>
                                        <td colSpan={2} className="py-4 text-muted">
                                            아직 매칭된 플레이어가 없습니다.
                                        </td>
                                    </tr>
                                )}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </>
    );
}