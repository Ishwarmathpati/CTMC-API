<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create ProductDiscount</name>
   <tag></tag>
   <elementGuidId>202a72f3-7405-4367-ba7d-7b3080d72df4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;value\&quot; : {\n    \&quot;type\&quot; : \&quot;absolute\&quot;,\n    \&quot;money\&quot; : [ {\n      \&quot;currencyCode\&quot; : \&quot;EUR\&quot;,\n      \&quot;centAmount\&quot; : 100\n    } ]\n  },\n  \&quot;predicate\&quot; : \&quot;1\u003d1\&quot;,\n  \&quot;name\&quot; : {\n    \&quot;en\&quot; : \&quot;test-discount1\&quot;\n  },\n  \&quot;description\&quot; : {\n    \&quot;en\&quot; : \&quot;test-discount1\&quot;\n  },\n  \&quot;isActive\&quot; : false,\n  \&quot;sortOrder\&quot; : \&quot;0.9534\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>ec79148c-2f2f-46ea-84e9-7c98a712d763</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>52600bf4-5389-4550-b653-bc278a023846</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/product-discounts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>342f18d5-7b4b-4719-9420-28dc91ea254c</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>585086e0-9c8d-4d16-9e7c-e93425aa26c2</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
