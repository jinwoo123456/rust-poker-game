import { API_URL } from "@/utils/path";
import { LoadingOverlay } from "@/utils/loading";
import axios from "axios";
import { useCallback, useEffect, useMemo, useState } from "react";
import Cookie from 'js-cookie';
import { useNavigate } from "react-router-dom";
type Player = {
    userid: string;
    status: number;
    match_at: string;
};

export default function MatchPage() {

    const [matchList, setMatchList] = useState<Player[]>([]);
    const [matchStatus] = useState(1);
    const [isLoading, setIsLoading] = useState(false);
    const [hasRedirected, setHasRedirected] = useState(false);
    const [isMatching, setIsMatching] = useState(false);
    const navigate = useNavigate();
    const userId = Cookie.get('userid');
    const getKstIsoString = () => {
        const now = new Date();
        const kst = new Date(now.getTime() + 9 * 60 * 60 * 1000);
        return kst.toISOString().replace('Z', '+09:00');
    };

    async function handleSoloMatch() {
        if (!userId) {
            console.error('로그인 정보가 없습니다.');
            return;
        }

        if (isMatching) {
            console.info('이미 매칭 진행 중입니다.');
            return;
        }
        try {
            setIsLoading(true);
            setIsMatching(true);
            navigate('/game', { state: userId });


        } catch (error) {
            console.error('매칭 생성 중 오류 발생:', error);
        } finally {
            setIsLoading(false);
            setIsMatching(false);
        }
    }
    async function handleStartGame(playerId: string) {

        if (!userId) {
            console.error('로그인 정보가 없습니다.');
            return;
        }

        try {
            setIsLoading(true);
            const resp = await axios.post(`${API_URL}/holdem/start`, {
                player_id: userId,
                select_player_id: playerId,
            });
            console.log('게임 시작 결과:', resp.data);
            setHasRedirected(true);
            navigate('/holdem', { state: resp.data });
        } catch (error) {
            console.error('게임 시작 중 오류 발생:', error);
        } finally {
            setIsLoading(false);
        }
    }


    async function handleMatch() {

        alert("현재 미구현 기능입니다. ai 대전을 이용해주세요.");
        return;

        if (!userId) {
            console.error('로그인 정보가 없습니다.');
            return;
        }

        if (isMatching) {
            console.info('이미 매칭 진행 중입니다.');
            return;
        }
        try {
            setIsLoading(true);
            setIsMatching(true);
            const resp = await axios.post(`${API_URL}/match/create`, {
                userid: userId,
                status: matchStatus,
                match_at: getKstIsoString(),
            });
            console.log('매칭 생성 결과:', resp.data);
            if (resp.data?.game_started && resp.data.game) {
                setHasRedirected(true);
                navigate('/holdem', { state: resp.data.game });
            } else {
                await getMatchList();
            }

        } catch (error) {
            console.error('매칭 생성 중 오류 발생:', error);
        } finally {
            setIsLoading(false);
            setIsMatching(false);
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
    const checkActiveGame = useCallback(async () => {
        if (hasRedirected || !userId) {
            return;
        }

        try {
            const resp = await axios.get(`${API_URL}/holdem/active/${userId}`);
            const data = resp.data;

            if (data?.success === 1 && data.game) {
                setHasRedirected(true);
                navigate('/holdem', { state: data.game });
            }
        } catch (error) {
            console.error('게임 상태 확인 중 오류 발생:', error);
        }
    }, [hasRedirected, navigate, userId]);

    useEffect(() => {
        let intervalId: ReturnType<typeof setInterval> | null = null;

        const startPolling = () => {
            if (intervalId) {
                clearInterval(intervalId);
            }
            intervalId = setInterval(() => {
                getMatchList();
                checkActiveGame();
            }, 5000);
        };

        getMatchList();
        checkActiveGame();
        console.log("매칭 페이지가 로드되었습니다.");
        startPolling();

        return () => {
            if (intervalId) {
                clearInterval(intervalId);
            }
        };
    }, [checkActiveGame, isMatching]);

    const filteredMatchList = useMemo(() => {
        if (!userId) {
            return matchList;
        }

        return matchList.filter((player) => player.userid !== userId);
    }, [matchList, userId]);

    return (
        <>
            <LoadingOverlay isLoading={isLoading} />
            <div className="container py-4">
                <div className="d-flex justify-content-between align-items-center mb-3">
                    <h1 className="h3 mb-0">매칭 페이지</h1>
                    <div style={{}}>
                        <button className="btn btn-primary" onClick={handleMatch}>
                            {isMatching ? '매칭 중...' : '매칭 잡기'}
                        </button>

                        <button className="btn btn-primary" onClick={handleSoloMatch}>
                            {isMatching ? '매칭 중...' : 'AI랑 하기'}
                        </button>
                    </div>
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
                                {filteredMatchList.length > 0 ? (
                                    filteredMatchList.map((player, index) => (
                                        <tr key={index}>
                                            <td className="fw-semibold">{player.userid}</td>
                                            <td>{new Date(player.match_at).toLocaleString('ko-KR', { timeZone: 'Asia/Seoul' })}</td>
                                            <td>
                                                <input type="hidden" value={player.userid} />
                                                <button onClick={() => handleStartGame(player.userid)} className="btn btn-sm btn-primary">게임하기</button>
                                            </td>
                                        </tr>
                                    ))
                                ) : (
                                    <tr>
                                        <td colSpan={3} className="py-4 text-muted">
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