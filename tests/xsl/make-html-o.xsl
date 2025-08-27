<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
  version="3.0">
  <xsl:output method="html"/>

  <xsl:template match="child::article">
    <HTML><BODY><xsl:apply-templates/></BODY></HTML>
  </xsl:template>
  <xsl:template match="child::sect1">
    <H1><xsl:apply-templates/></H1>
  </xsl:template>
  <xsl:template match="child::sect2">
    <H2><xsl:apply-templates/></H2>
  </xsl:template>
  <xsl:template match="child::para">
    <P><xsl:apply-templates/></P>
  </xsl:template>
  <xsl:template match="child::emph">
    <xsl:choose>
      <xsl:when test="attribute::role eq 'strong'">
	<B><xsl:apply-templates/></B>
      </xsl:when>
      <xsl:when test="attribute::role eq 'underline'">
	<U><xsl:apply-templates/></U>
      </xsl:when>
      <xsl:otherwise>
	<EM><xsl:apply-templates/></EM>
      </xsl:otherwise>
    </xsl:choose>
  </xsl:template>
</xsl:stylesheet>
