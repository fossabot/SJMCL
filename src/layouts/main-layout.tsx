import { Card, Center, Flex } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { BeatLoader } from "react-spinners";
import HeadNavBar from "@/components/head-navbar";
import { useLauncherConfig } from "@/contexts/config";

interface MainLayoutProps {
  children: React.ReactNode;
}

const MainLayout = ({ children }: MainLayoutProps) => {
  const router = useRouter();
  const { config } = useLauncherConfig();
  const isNavOnLeft =
    config.appearance.theme.headNavStyle === "simplified-left";

  if (router.pathname.startsWith("/standalone")) return children;

  if (config.mocked)
    return (
      <Center h="100%">
        <BeatLoader size={16} color="gray" />
      </Center>
    );

  return (
    <Flex
      direction={isNavOnLeft ? "row" : "column"}
      h="100vh"
      bgImg={`url('/images/backgrounds/${config.appearance.background.presetChoice}.jpg')`}
      bgSize="cover"
      bgPosition="center"
      bgRepeat="no-repeat"
    >
      <HeadNavBar />
      {router.pathname === "/launch" ? (
        <>{children}</>
      ) : (
        <Card
          className="content-blur-bg"
          h={isNavOnLeft ? "inherit" : "100%"}
          overflow="auto"
          mt={1}
          mb={4}
          mx={4}
        >
          {children}
        </Card>
      )}
    </Flex>
  );
};

export default MainLayout;
